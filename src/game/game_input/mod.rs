use bevy::prelude::*;
use crate::prelude::*;

pub mod in_game_mouse_input;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InGameMouseInputPlugin);
    }
}