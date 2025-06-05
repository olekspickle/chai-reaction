use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default()).add_systems(
        Update,
        (spawn_particles, despawn_particles).run_if(in_state(Screen::Gameplay)),
    );
    //.add_observer(start_emitting);
}

#[derive(Component, Debug, Clone, Reflect, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticleKind {
    Water,
    Fire,
}
#[derive(Component)]
pub struct WaterDrop;
#[derive(Component)]
pub struct Spark;
#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
}
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct ParticleEmitter {
    kind: ParticleKind,
    spawn_rate: f32,                 // Particles per second
    spawn_timer: Timer,              // Timer to control spawn rate
    initial_speed_range: (f32, f32), // Min and max initial speed
    initial_angle_deg_range: (f32, f32),
    particle_gravity_scale: f32, // How much Avian's gravity affects the particle
    particle_lifetime_s: f32, // Angle range in degrees for the initial velocity direction (0 is right, 90 is up)
}

impl ParticleEmitter {
    pub fn new(
        kind: ParticleKind,
        spawn_rate: f32,
        initial_speed_min: f32,
        initial_speed_max: f32,
        initial_angle_deg_min: f32,
        initial_angle_deg_max: f32,
        particle_lifetime_s: f32,
        particle_gravity_scale: f32,
    ) -> Self {
        Self {
            kind,
            spawn_rate,
            particle_lifetime_s,
            spawn_timer: Timer::from_seconds(1.0 / spawn_rate, TimerMode::Repeating),
            initial_speed_range: (initial_speed_min, initial_speed_max),
            initial_angle_deg_range: (initial_angle_deg_min, initial_angle_deg_max),
            particle_gravity_scale,
        }
    }
}

fn spawn_particles(
    time: Res<Time>,
    cfg: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut emitter: Query<(&mut ParticleEmitter, &GlobalTransform)>,
    droplet_count_query: Query<&ParticleKind>,
) {
    let mut rng = rand::thread_rng();

    let max_particles = cfg.physics.water.max_particles as usize;
    let current_droplet_count: usize = droplet_count_query
        .iter()
        .filter(|p| **p == ParticleKind::Water)
        .collect::<Vec<&ParticleKind>>()
        .len();

    for (mut emitter, global_transform) in emitter.iter_mut() {
        emitter.spawn_timer.tick(time.delta());

        if emitter.spawn_timer.just_finished() {
            let mut particles_to_spawn =
                (emitter.spawn_rate * emitter.spawn_timer.duration().as_secs_f32()).max(1.0) as u32;

            if current_droplet_count >= max_particles {
                particles_to_spawn = 0; // Don't spawn any if at or above limit
            // println!("Particle limit reached ({}/{}). Not spawning.", current_droplet_count, max_particles);
            } else if current_droplet_count + (particles_to_spawn as usize) > max_particles {
                // If spawning all would exceed the limit, spawn only enough to reach the limit
                particles_to_spawn = (max_particles - current_droplet_count) as u32;
                // println!("Approaching limit. Spawning {} more particles to reach {}.", particles_to_spawn, max_particles);
            }

            // Only proceed if there are particles to spawn after the check
            if particles_to_spawn == 0 {
                continue; // Skip to the next emitter if no particles to spawn
            }
            let particles_to_spawn =
                (emitter.spawn_rate * emitter.spawn_timer.duration().as_secs_f32()).max(1.0) as u32;
            let spawn_position = global_transform.translation().truncate();

            for _ in 0..particles_to_spawn {
                // Randomize initial speed and angle
                let speed =
                    rng.gen_range(emitter.initial_speed_range.0..=emitter.initial_speed_range.1);
                let angle_rad = rng.gen_range(
                    emitter.initial_angle_deg_range.0.to_radians()
                        ..=emitter.initial_angle_deg_range.1.to_radians(),
                );

                let initial_velocity = Vec2::new(angle_rad.cos() * speed, angle_rad.sin() * speed);

                let mesh = meshes.add(Circle::new(cfg.droplet_radius));
                let (color, marker) = match emitter.kind {
                    ParticleKind::Water => (WATER, ParticleKind::Water),
                    ParticleKind::Fire => (FIRE, ParticleKind::Fire),
                };
                let material = materials.add(color);
                commands.spawn((
                    marker,
                    Mesh2d(mesh),
                    MeshMaterial2d(material),
                    Transform::from_translation(spawn_position.extend(0.0)),
                    // --- Avian2D Physics Components ---
                    RigidBody::Dynamic,
                    Collider::circle(cfg.droplet_radius),
                    LinearVelocity(initial_velocity),
                    GravityScale(emitter.particle_gravity_scale),
                    Friction::new(cfg.physics.water.friction)
                        .with_combine_rule(CoefficientCombine::Multiply),
                    Restitution::new(cfg.physics.water.restitution)
                        .with_combine_rule(CoefficientCombine::Min), // How bounciness is combined
                    // Mass(0.1),
                    SleepingDisabled,
                    Particle {
                        lifetime: Timer::from_seconds(emitter.particle_lifetime_s, TimerMode::Once),
                    },
                ));
            }
        }
    }
}

fn despawn_particles(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Particle)>, // Added Name for logging
) {
    for (entity, t, particle) in query.iter() {
        if particle.lifetime.finished()
            || t.translation.y < -1000.0
            || t.translation.x < -1000.0
            || t.translation.x > 1000.0
        {
            commands.entity(entity).despawn();
        }
    }
}

fn start_emitting(
    _: Trigger<OnGlassHit>,
    mut commands: Commands,
    glass: Query<Entity, With<Glass>>,
) {
    for e in glass.iter() {
        commands.entity(e).insert((
            StateScoped(GameLevel::Sink),
            ParticleEmitter::new(
                ParticleKind::Water,
                10.0,  // Spawn 10 particles per second
                20.0,  // Min initial speed
                50.0,  // Max initial speed
                45.0,  // Min angle (degrees, e.g., 60 = upwards right)
                160.0, // Max angle (degrees, e.g., 120 = upwards left)
                10.0,  // live for 10s
                1.0,   // Normal gravity effect
            ),
        ));
    }
}
