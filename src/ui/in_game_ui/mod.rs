
use crate::prelude::*;
use bevy::prelude::*;
use crate::prelude::machines_spawn_area::MachinesSpawnAreaPlugin;


pub mod machines_spawn_area;


pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MachinesSpawnAreaPlugin);
    }
}