use crate::prelude::Val::Percent;
use crate::prelude::*;
use bevy::{prelude::*, ui::Val::*};

pub struct MachinePartToSpawnButtonsPlugin;

impl Plugin for MachinePartToSpawnButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_part_picking_buttons
                .run_if(in_state(Screen::Gameplay).and(resource_exists_and_changed::<LoadedLevel>)),
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
) {
    for entity in &existing_area {
        commands.entity(entity).despawn();
    }

    let config = level_configs.get(&loaded_level.0).unwrap();

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
            );
            buttons.push(commands.spawn(button_bundle).id());
        }
    }

    commands
        .spawn((
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
            parent.spawn((
                StateScoped(Screen::Gameplay),
                btn_sq("Remove", set_delete_mode),
            ));
        });
}

fn btn_with_machine_part_type(part_type: MachinePartType, text: impl Into<String>) -> impl Bundle {
    (
        StateScoped(Screen::Gameplay),
        part_type,
        btn_sq(text.into(), set_picked_machine_part),
    )
}

fn set_picked_machine_part(
    trigger: Trigger<Pointer<Pressed>>,
    mut picking_state: ResMut<PickingState>,
    part_types: Query<&MachinePartType>,
    child_ofs: Query<&ChildOf>,
) {
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
