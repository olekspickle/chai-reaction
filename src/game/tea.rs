use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (apply_tea, update_tea_sensors)
            .run_if(resource_exists::<Config>)
            .run_if(resource_exists::<AudioSources>),
    );
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Tea;
#[derive(Default, Component, Copy, Clone, PartialEq)]
pub struct TeaSensor(pub Recipe);
#[derive(Component)]
pub struct Satisfied;

#[derive(Component, Debug, Default, Copy, Clone, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Recipe {
    milky: bool,
    sweet: bool,
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

fn update_tea_sensors(
    collisions: Collisions,
    mut commands: Commands,
    tea_sensors: Query<(Entity, &TeaSensor)>,
    particles: Query<(Entity, &Particle)>,
) {
    for (sensor_entity, sensor) in &tea_sensors {
        let mut total = ParticleContents::default();
        let mut count = 0;
        for (particle_entity, particle) in &particles {
            if collisions.contains(sensor_entity, particle_entity) {
                total = total + particle.contents;
                count += 1;
            }
        }

        commands.entity(sensor_entity).remove::<Satisfied>();
        if count >= 10 {
            let avg = total / count as f32;
            if avg.is_tea() && avg.is_milky() == sensor.0.milky && avg.is_sweet() == sensor.0.sweet
            {
                commands.entity(sensor_entity).insert(Satisfied);
            }
        }
    }
}
