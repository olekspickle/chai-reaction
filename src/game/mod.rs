use crate::prelude::*;
use bevy::prelude::*;

pub mod camera;
pub mod currency;
pub mod game_level;
pub mod input_dispatch;
pub mod levels;
pub mod machine_parts;
pub mod settings;
pub mod sound;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        settings::plugin,
        camera::plugin,
        levels::plugin,
        sound::plugin,
        input_dispatch::plugin,
    ))
    .add_plugins((
        GameLevelPlugin,
        MachinePartsPlugin,
        CurrencyPlugin,
        GameInputPlugin,
    ));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);
