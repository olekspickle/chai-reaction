use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (apply_tea, count_tea)
            .run_if(resource_exists::<Config>)
            .run_if(resource_exists::<AudioSources>),
    );
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Tea;
#[derive(Default, Component, Copy, Clone, PartialEq)]
pub struct TeaCounter(pub u32);

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Recipe {
    sugar: f32,
    milk: f32,
}

fn apply_tea(
    collisions: Collisions,
    tea: Query<Entity, With<Tea>>,
    mut particles: Query<(Entity, &mut Particle)>,
    config: Res<Config>,
    time: Res<Time>,
) {
    for tea_entity in &tea {
        for (particle_entity, mut particle) in &mut particles {
            if particle.contents.heat < config.physics.brewing_temperature {
                continue;
            }
            if collisions.contains(tea_entity, particle_entity) {
                particle.contents.tea += time.delta().as_secs_f32();
            }
        }
    }
}

fn count_tea(
    collisions: Collisions,
    settings: Res<Settings>,
    audio_sources: Res<AudioSources>,
    mut commands: Commands,
    mut counter: Query<(Entity, &mut TeaCounter)>,
    mut particles: Query<(Entity, &Particle)>,
    mut score: ResMut<Score>,
) {
    for (counter_entity, mut counter) in &mut counter {
        for (particle_entity, particle) in &particles {
            if !particle.is_tea() {
                let vol = settings.sound.general * settings.sound.sfx;
                // commands.spawn(sfx(audio_sources.cup_drop_brewed.clone(), vol));
                continue;
            }
            if collisions.contains(counter_entity, particle_entity) {
                let vol = settings.sound.general * settings.sound.sfx;
                // commands.spawn(sfx(audio_sources.cup_drop.clone(), vol));
                counter.0 += 1;
                score.0 += 1;
            }
        }
    }
}
