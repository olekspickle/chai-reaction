use crate::prelude::Val::Percent;
use crate::{game::physics::PhysicsState, prelude::*};
use bevy::{prelude::*, ui::Val::*};

pub struct MachinePartToSpawnButtonsPlugin;

impl Plugin for MachinePartToSpawnButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_part_picking_buttons
                .run_if(in_state(Screen::Gameplay).and(resource_exists_and_changed::<LoadedLevel>)),
        )
        .add_systems(
            OnEnter(PhysicsState::Paused),
            enable_machine_part_type_buttons,
        )
        .add_systems(
            OnEnter(PhysicsState::Running),
            disable_machine_part_type_buttons,
        );
    }
}

#[derive(Component)]
struct MachinePartButtonNode;

fn spawn_part_picking_buttons(
    mut commands: Commands,
    machine_parts: Res<MachinePartConfigByType>,
    loaded_level: Res<LoadedLevel>,
    level_configs: Res<Assets<LevelConfig>>,
    existing_area: Query<Entity, With<MachinePartButtonNode>>,
    editor_mode: Res<EditorMode>,
    machine_part_configs: Res<MachinePartConfigByType>,
) {
    for entity in &existing_area {
        commands.entity(entity).despawn();
    }

    let config = level_configs.get(&loaded_level.0).unwrap();
    let level = GameLevel::Loaded(config.name.clone());

    let mut buttons = Vec::new();
    for part in machine_parts.0.keys() {
        if editor_mode.0 || config.available_machine_parts.contains(part) {
            // You need to provide the correct PlacementContext value for each part.
            let button_bundle = btn_with_machine_part_type(
                MachinePartType {
                    name: part.clone(),
                    context: PlacementContext::default(),
                }, // Replace PlacementContext::default() with the correct context if needed
                part,
                level.clone(),
                &machine_part_configs,
            );
            buttons.push(commands.spawn(button_bundle).id());
        }
    }

    commands
        .spawn((
            StateScoped(level),
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                width: Percent(100.0),
                flex_wrap: FlexWrap::Wrap,
                position_type: PositionType::Absolute,
                bottom: Px(0.0),
                ..default()
            },
            MachinePartButtonNode,
        ))
        .add_children(&buttons)
        .with_children(|parent| {
            parent.spawn(btn_sq("Remove", set_delete_mode));
        });
}

fn disable_machine_part_type_buttons(
    mut commands: Commands,
    buttons: Query<Entity, With<MachinePartButton>>,
    children: Query<&Children>,
) {
    println!("DISABLING");
    for entity in &buttons {
        println!("DISABLED");
        for entity in [entity]
            .into_iter()
            .chain(children.iter_descendants(entity))
        {
            commands.entity(entity).insert(DisabledButton);
        }
    }
}

fn enable_machine_part_type_buttons(
    mut commands: Commands,
    buttons: Query<Entity, With<MachinePartButton>>,
    children: Query<&Children>,
) {
    for entity in &buttons {
        for entity in [entity]
            .into_iter()
            .chain(children.iter_descendants(entity))
        {
            commands.entity(entity).remove::<DisabledButton>();
        }
    }
}

#[derive(Component)]
pub struct MachinePartButton;

fn btn_with_machine_part_type(
    part_type: MachinePartType,
    text: impl Into<String>,
    level: GameLevel,
    machine_part_configs: &Res<MachinePartConfigByType>,
) -> impl Bundle {
    if let Some(image) = try_fetch_button_image(part_type.clone(), machine_part_configs) {
        (
            StateScoped(level),
            MachinePartButton,
            part_type,
            btn_sq(Opts::new(image), set_picked_machine_part),
        )
    } else {
        (
            StateScoped(level),
            MachinePartButton,
            part_type,
            btn_sq(Opts::new(text.into()), set_picked_machine_part),
        )
    }
}

fn try_fetch_button_image(
    part_type: MachinePartType,
    machine_part_configs: &Res<MachinePartConfigByType>,
) -> Option<Handle<Image>> {
    if let Some(config) = machine_part_configs.0.get(&part_type.name) {
        for subassembly in &config.subassemblies {
            if let SubAssembly::Sprite { sprite, .. } = subassembly {
                return Some(sprite.image.clone());
            }
        }
    }
    None
}

fn set_picked_machine_part(
    trigger: Trigger<Pointer<Pressed>>,
    physics_state: Res<State<PhysicsState>>,
    mut picking_state: ResMut<PickingState>,
    part_types: Query<&MachinePartType>,
    child_ofs: Query<&ChildOf>,
) {
    if *physics_state.get() == PhysicsState::Running {
        return;
    }
    if let Ok(child_of) = child_ofs.get(trigger.target()) {
        if let Ok(part) = part_types.get(child_of.0) {
            *picking_state = PickingState::Placing(part.clone());
        }
    }
}

fn set_delete_mode(
    trigger: Trigger<Pointer<Pressed>>,
    mut picking_state: ResMut<PickingState>,
    child_ofs: Query<&ChildOf>,
) {
    if child_ofs.contains(trigger.target()) {
        *picking_state = PickingState::Erasing;
    }
}
