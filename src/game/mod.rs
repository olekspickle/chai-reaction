use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub mod camera;
pub mod conveyor_belts;
pub mod currency;
pub mod heat;
pub mod input_dispatch;
pub mod levels;
pub mod machine_parts;
pub mod particles;
pub mod physics;
pub mod settings;
pub mod sound;
pub mod tea;
pub mod victory;
pub mod tea_particles;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        settings::plugin,
        physics::plugin,
        camera::plugin,
        sound::plugin,
        input_dispatch::plugin,
        particles::plugin,
        levels::plugin,
        heat::plugin,
        tea::plugin,
        victory::plugin,
        conveyor_belts::plugin,
        tea_particles::plugin,
    ))
    .add_plugins((MachinePartsPlugin, CurrencyPlugin));
}

#[derive(PhysicsLayer, Default)]
pub enum ParticleLayer {
    #[default]
    Default,
    Fluid,
    TeaLeaves,
}
