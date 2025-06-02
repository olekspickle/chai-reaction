use crate::prelude::*;
use bevy::prelude::*;

pub mod machine_part_type;
pub mod machine_part_config;
pub mod machine_part_config_by_type;
pub mod events;
pub mod machine_part_spawner;
pub mod consts;

pub struct MachinePartsPlugin;

impl Plugin for MachinePartsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MachinePartConfigByTypePlugin, MachinePartEventsPlugin, MachinePartSpawnerPlugin));
    }
}