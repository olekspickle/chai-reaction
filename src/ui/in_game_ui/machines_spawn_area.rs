use crate::prelude::*;
use bevy::prelude::*;
use crate::prelude::tags::MachinesSpawnArea;

pub struct MachinesSpawnAreaPlugin;

impl Plugin for MachinesSpawnAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), (spawn_machines_area, spawn_preview))
            .add_systems(Update, (
                change_preview_sprite.run_if(resource_exists_and_changed::<PickingState>),
                change_preview_visibility.run_if(resource_exists_and_changed::<PickingState>),
            ))
            .add_observer(on_machine_spawn_area_click)
            .add_observer(preview_machine_spawn);
    }
}

fn spawn_machines_area(
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
){
    let maybe_window = windows.single();
    if let Ok(window) = maybe_window {
        let square = meshes.add(Rectangle::new(window.resolution.width(), window.resolution.height()));
        let no_color = materials.add(Color::NONE);
        commands.spawn((
            StateScoped(Screen::Gameplay),
            MachinesSpawnArea,
            Mesh2d(square),
            MeshMaterial2d(no_color),
        ));
    }
}

#[derive(Component)]
struct MachinePartPreview;

fn spawn_preview(
    mut commands: Commands,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
) {
    commands.spawn((
        MachinePartPreview,
        Sprite{
            image: machine_part_config_by_type.0.values().next().unwrap().sprite.clone(),
            color: Color::srgba(1.0, 1.0, 1.0, 0.5),
            ..default()
        },
        Visibility::Hidden,
    ));
}

fn on_machine_spawn_area_click(
    trigger: Trigger<Pointer<Click>>,
    mut machine_part_request_writer: EventWriter<MachinePartRequest>,
    machine_spawn_areas: Query<(), With<MachinesSpawnArea>>,
    picking_state: Res<PickingState>,
){
    if let PickingState::Placing(ty) = *picking_state {
        if machine_spawn_areas.contains(trigger.target()){
            if let Some(hit_position) = trigger.hit.position {
                machine_part_request_writer.write(MachinePartRequest::SpawnMachinePart(
                    MachinePartSpawnRequest{
                        location: ((hit_position/MACHINE_PARTS_GRID_SCALE).round() * MACHINE_PARTS_GRID_SCALE).with_z(MACHINE_PARTS_BASIC_Z_LAYER),
                        part_type: ty,
                    }
                ));
            }
        }
    }
}

fn preview_machine_spawn(
    trigger: Trigger<Pointer<Move>>,
    machine_spawn_areas: Query<(), With<MachinesSpawnArea>>,
    mut preview: Single<&mut Transform, With<MachinePartPreview>>,
){
    if machine_spawn_areas.contains(trigger.target()){
        if let Some(hit_position) = trigger.hit.position {
            preview.translation = ((hit_position/MACHINE_PARTS_GRID_SCALE).round() * MACHINE_PARTS_GRID_SCALE).with_z(MACHINE_PARTS_PREVIEW_Z_LAYER);
        }
    }
}

fn change_preview_sprite(
    picking_state: Res<PickingState>,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
    mut preview: Single<&mut Sprite, With<MachinePartPreview>>,
) {
    if let PickingState::Placing(ty) = &*picking_state {
        if let Some(part_config) = machine_part_config_by_type.0.get(ty){
            preview.image = part_config.sprite.clone();
        }
    }
}

fn change_preview_visibility(
    picking_state: Res<PickingState>,
    mut preview: Single<&mut Visibility, With<MachinePartPreview>>,
) {
    if matches!(*picking_state, PickingState::Placing(_)) {
        **preview = Visibility::Inherited;
    } else {
        **preview = Visibility::Hidden;
    }
}
