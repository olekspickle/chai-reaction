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

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
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
    ))
    .add_systems(OnEnter(Screen::Gameplay), clear_score)
    .add_plugins((MachinePartsPlugin, CurrencyPlugin));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);

fn clear_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

#[derive(PhysicsLayer, Default)]
pub enum ParticleLayer {
    #[default]
    Default,
    Fluid,
    TeaLeaves,
}
