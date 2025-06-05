use crate::prelude::*;
use bevy::prelude::*;

pub mod camera;
pub mod currency;
pub mod heat;
pub mod input_dispatch;
pub mod levels;
pub mod machine_parts;
pub mod particles;
pub mod settings;
pub mod sound;
pub mod tea;
pub mod victory;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        settings::plugin,
        camera::plugin,
        sound::plugin,
        input_dispatch::plugin,
        particles::plugin,
        levels::plugin,
        heat::plugin,
        tea::plugin,
        victory::plugin,
    ))
    .add_plugins((MachinePartsPlugin, CurrencyPlugin));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);
