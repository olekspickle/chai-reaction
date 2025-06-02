use bevy::prelude::*;
use crate::prelude::*;

pub mod camera;
pub mod input_dispatch;
pub mod scene;
pub mod settings;
pub mod sound;
pub mod machine_parts;
pub mod currency;
pub mod game_level;
pub mod game_input;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        settings::plugin,
        camera::plugin,
        scene::plugin,
        sound::plugin,
        input_dispatch::plugin,
    ))
        .add_plugins((GameLevelPlugin, MachinePartsPlugin, CurrencyPlugin, GameInputPlugin));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);
