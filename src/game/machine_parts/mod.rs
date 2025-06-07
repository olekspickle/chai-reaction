use crate::{game::machine_parts::animator::AnimatorPlugin, prelude::*};
use bevy::prelude::*;

pub mod animator;
pub mod consts;
pub mod events;
pub mod flow_field;
pub mod machine_part_config;
pub mod machine_part_config_by_type;
pub mod machine_part_spawner;
pub mod machine_part_type;
pub mod picked_machine_part_type;

pub struct MachinePartsPlugin;

impl Plugin for MachinePartsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MachinePartConfigByTypePlugin,
            MachinePartEventsPlugin,
            MachinePartSpawnerPlugin,
            PickedMachinePartTypePlugin,
            
            FlowFieldPlugin,
            AnimatorPlugin
        ));
    }
}
