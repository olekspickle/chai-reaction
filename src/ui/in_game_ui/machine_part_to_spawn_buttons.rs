use crate::prelude::Val::Percent;
use crate::prelude::*;
use bevy::prelude::*;

pub struct MachinePartToSpawnButtonsPlugin;

impl Plugin for MachinePartToSpawnButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), spawn_part_picking_buttons);
    }
}

fn spawn_part_picking_buttons(mut commands: Commands, machine_parts: Res<MachinePartConfigByType>) {
    let mut buttons = Vec::new();
    for part in machine_parts.0.keys() {
        let button_bundle = btn_with_machine_part_type(MachinePartType(part.clone()), part);
        buttons.push(commands.spawn(button_bundle).id());
    }
    let mut node_commands = commands.spawn((Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceEvenly,
        width: Percent(50.0),
        ..default()
    },));
    node_commands.add_children(&buttons);
    node_commands.with_children(|parent| {
        parent.spawn((
            StateScoped(Screen::Gameplay),
            btn("Remove", set_delete_mode),
        ));
    });
}

fn btn_with_machine_part_type(part_type: MachinePartType, text: impl Into<String>) -> impl Bundle {
    (
        StateScoped(Screen::Gameplay),
        part_type,
        btn(text.into(), set_picked_machine_part),
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
