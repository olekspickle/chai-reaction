
use crate::prelude::*;
use bevy::prelude::*;
use crate::prelude::machine_part_to_spawn_buttons::MachinePartToSpawnButtonsPlugin;
use crate::prelude::machines_spawn_area::MachinesSpawnAreaPlugin;


pub mod machines_spawn_area;
pub mod machine_part_to_spawn_buttons;

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MachinesSpawnAreaPlugin, MachinePartToSpawnButtonsPlugin));
    }
}