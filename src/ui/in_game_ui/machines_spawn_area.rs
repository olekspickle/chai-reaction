use crate::prelude::*;
use bevy::prelude::*;
use crate::prelude::tags::MachinesSpawnArea;

pub struct MachinesSpawnAreaPlugin;

impl Plugin for MachinesSpawnAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), spawn_machines_area)
            .add_systems(OnExit(Screen::Gameplay), despawn_machines_area)
            .add_observer(on_machine_spawn_area_click);
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
            MachinesSpawnArea,
            Mesh2d(square),
            MeshMaterial2d(no_color),
        ));
    }
}

fn despawn_machines_area(
    machine_spawn_areas: Query<Entity, With<MachinesSpawnArea>>,
    mut commands: Commands,
){
    for area in &machine_spawn_areas{
        if let Ok(mut entity_commands) = commands.get_entity(area){
            entity_commands.despawn();
        }
    }
}

fn on_machine_spawn_area_click(
    trigger: Trigger<Pointer<Click>>,
    mut machine_part_request_writer: EventWriter<MachinePartRequest>,
    machine_spawn_areas: Query<(), With<MachinesSpawnArea>>,
){
    if machine_spawn_areas.contains(trigger.target()){
        if let Some(hit_position) = trigger.hit.position{
            machine_part_request_writer.write(MachinePartRequest::SpawnMachinePart(
                MachinePartSpawnRequest{
                    location: hit_position.with_z(MACHINE_PARTS_BASIC_Z_LAYER),
                    part_type: MachinePartType::Scale
                }
            ));
        }
    }
}