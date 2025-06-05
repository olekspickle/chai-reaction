use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (apply_tea, count_tea).run_if(resource_exists::<Config>),
    );
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Tea;
#[derive(Default, Component, Copy, Clone, PartialEq)]
pub struct TeaCounter(pub u32);

fn apply_tea(
    collisions: Collisions,
    tea: Query<Entity, With<Tea>>,
    mut particles: Query<(Entity, &Particle, &mut ParticleKind)>,
    config: Res<Config>,
) {
    for tea_entity in &tea {
        for (particle_entity, particle, mut kind) in &mut particles {
            if *kind != ParticleKind::Water || particle.heat < config.physics.brewing_temperature {
                continue;
            }
            if collisions.contains(tea_entity, particle_entity) {
                *kind = ParticleKind::BrewedTea;
            }
        }
    }
}

fn count_tea(
    collisions: Collisions,
    mut counter: Query<(Entity, &mut TeaCounter)>,
    mut particles: Query<(Entity, &Particle, &mut ParticleKind)>,
) {
    for (counter_entity, mut counter) in &mut counter {
        for (particle_entity, particle, mut kind) in &mut particles {
            if *kind != ParticleKind::BrewedTea {
                continue;
            }
            if collisions.contains(counter_entity, particle_entity) {
                counter.0 += 1;
            }
        }
    }
}
