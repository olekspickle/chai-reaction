use bevy::prelude::*;

pub mod machine_part_to_spawn_buttons;
pub mod machines_spawn_area;

use machine_part_to_spawn_buttons::*;
use machines_spawn_area::*;

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MachinesSpawnAreaPlugin, MachinePartToSpawnButtonsPlugin));
    }
}
