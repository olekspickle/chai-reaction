use crate::{game::ParticleLayer, prelude::*};
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            despawn_particles,
            // trigger_fluid_filter_buttons,
            (spawn_particles, recolor_particles, mix_particles)
                .before(crate::game::levels::prepare_levels),
        )
            .run_if(in_state(Screen::Gameplay)),
    );
    // .add_observer(activate_fluid_filter)
    // .add_observer(deactivate_fluid_filter)
}

#[derive(Component)]
pub struct WaterDrop;
#[derive(Component)]
pub struct Spark;
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct Particle {
    pub lifetime: Timer,
    pub contents: ParticleContents,
}

impl ParticleContents {
    pub fn is_tea(&self) -> bool {
        self.tea > 0.5
    }

    pub fn is_milky(&self) -> bool {
        self.milk > 0.5
    }

    pub fn is_sweet(&self) -> bool {
        self.sugar > 0.5
    }

    pub fn clamp(&mut self) {
        self.heat = self.heat.clamp(0.0, 1.0);
        self.tea = self.tea.clamp(0.0, 1.0);
        self.sugar = self.sugar.clamp(0.0, 1.0);
        self.milk = self.milk.clamp(0.0, 1.0);
    }
}

#[derive(Default, Debug, Copy, Clone, Reflect, Serialize, Deserialize)]
pub struct ParticleContents {
    #[serde(default = "default_heat")]
    pub heat: f32,
    #[serde(default)]
    pub tea: f32,
    #[serde(default)]
    pub sugar: f32,
    #[serde(default)]
    pub milk: f32,
}
fn default_heat() -> f32 {
    1.0
}

impl std::ops::Add for ParticleContents {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            heat: self.heat + other.heat,
            tea: self.tea + other.tea,
            sugar: self.sugar + other.sugar,
            milk: self.milk + other.milk,
        }
    }
}

impl std::ops::Div<f32> for ParticleContents {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            heat: self.heat / other,
            tea: self.tea / other,
            sugar: self.sugar / other,
            milk: self.milk / other,
        }
    }
}

impl std::ops::Mul<f32> for ParticleContents {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            heat: self.heat * other,
            tea: self.tea * other,
            sugar: self.sugar * other,
            milk: self.milk * other,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct ParticleEmitter {
    kind: ParticleContents,
    spawn_rate: f32,                 // Particles per second
    spawn_timer: Timer,              // Timer to control spawn rate
    initial_speed_range: (f32, f32), // Min and max initial speed
    initial_angle_deg_range: (f32, f32),
    particle_gravity_scale: f32, // How much Avian's gravity affects the particle
    particle_lifetime_s: f32, // Angle range in degrees for the initial velocity direction (0 is right, 90 is up)
}

impl ParticleEmitter {
    pub fn new(
        kind: ParticleContents,
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
    droplet_count_query: Query<&Particle>,
    editor_mode: Res<EditorMode>,
) {
    if editor_mode.0 {
        return;
    }
    let mut rng = rand::thread_rng();

    let max_particles = cfg.physics.water.max_particles as usize;
    let current_droplet_count: usize = droplet_count_query.iter().count();

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

                let contents = emitter.kind;

                let material = materials.add(WATER);
                commands.spawn((
                    LevelObject,
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
                    Mass(0.1),
                    CollisionLayers::new(
                        ParticleLayer::Fluid,
                        [
                            ParticleLayer::Default,
                            ParticleLayer::Fluid,
                            ParticleLayer::TeaLeaves,
                        ],
                    ),
                    //SleepingDisabled,
                    Particle {
                        lifetime: Timer::from_seconds(emitter.particle_lifetime_s, TimerMode::Once),
                        contents,
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

fn recolor_particles(
    mut commands: Commands,
    particles: Query<(Entity, &Particle), Changed<Particle>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, particle) in &particles {
        let color = WATER
            .mix(&BREWED_TEA, particle.contents.tea.min(1.0))
            .lighter(particle.contents.milk.min(1.0));
        /*
        let water = WATER.to_linear();
        let brewed_tea = BREWED_TEA.to_linear();
        let total_stuff = 1.0 + particle.contents.tea + particle.contents.milk;
        let r = (water.red + brewed_tea.red * particle.contents.tea + particle.contents.milk)
            / total_stuff;
        let g = (water.green + brewed_tea.green * particle.contents.tea + particle.contents.milk)
            / total_stuff;
        let b = (water.blue + brewed_tea.blue * particle.contents.tea + particle.contents.milk)
            / total_stuff;
        let color = Color::linear_rgba(r, g, b, 1.0);
        */
        commands
            .entity(entity)
            .insert(MeshMaterial2d(materials.add(color)));
    }
}

fn mix_particles(
    mut particles: Query<(Entity, &mut Particle)>,
    collisions: Collisions,
    time: Res<Time>,
) {
    let d = (time.delta().as_secs_f32() * 40.0).min(1.0);
    let entities: Vec<_> = particles.iter().map(|(e, _)| e).collect();
    for entity in entities {
        for other in collisions.entities_colliding_with(entity) {
            if let Ok([(_, mut src_particle), (_, dst_particle)]) =
                particles.get_many_mut([entity, other])
            {
                let avg = (src_particle.contents + dst_particle.contents) / 2.0;
                src_particle.contents = src_particle.contents * (1.0 - d) + avg * d;
                src_particle.contents.clamp();
            }
        }
    }
}

#[derive(Event)]
#[event(auto_propagate)]
pub struct ActivateFluidFilter;
#[derive(Event)]
#[event(auto_propagate)]
pub struct DeactivateFluidFilter;

#[derive(Component)]
pub struct FluidFilter;
#[derive(Default, Component)]
pub struct FluidFilterButton(pub bool);

// fn activate_fluid_filter(
//     trigger: Trigger<ActivateFluidFilter>,
//     mut commands: Commands,
//     filters: Query<&FluidFilter>,
// ) {
//     if filters.contains(trigger.target()) {
//         commands
//             .entity(trigger.target())
//             .insert(CollisionLayers::new(
//                 ParticleLayer::Default,
//                 [ParticleLayer::Default, ParticleLayer::TeaLeaves],
//             ));
//     }
// }

// fn deactivate_fluid_filter(
//     trigger: Trigger<DeactivateFluidFilter>,
//     mut commands: Commands,
//     filters: Query<&FluidFilter>,
// ) {
//     if filters.contains(trigger.target()) {
//         commands
//             .entity(trigger.target())
//             .insert(CollisionLayers::new(
//                 ParticleLayer::Default,
//                 [
//                     ParticleLayer::Default,
//                     ParticleLayer::TeaLeaves,
//                     ParticleLayer::Fluid,
//                 ],
//             ));
//     }
// }

// fn trigger_fluid_filter_buttons(
//     mut commands: Commands,
//     collisions: Collisions,
//     mut buttons: Query<(Entity, &ChildOf, &mut FluidFilterButton)>,
//     children: Query<&Children>,
//     filters: Query<Entity, With<FluidFilter>>,
// ) {
//     for (button_entity, parent, mut button) in &mut buttons {
//         let mut triggered = false;
//         let cs: Vec<_> = collisions.entities_colliding_with(button_entity).collect();
//         if !cs.is_empty() {
//             if !button.0 {
//                 for entity in children.iter_descendants(parent.0) {
//                     if filters.contains(entity) {
//                         commands.entity(entity).trigger(ActivateFluidFilter);
//                     }
//                 }
//                 button.0 = true;
//             }
//             triggered = true;
//         }
//         if !triggered && button.0 {
//             for entity in children.iter_descendants(parent.0) {
//                 if filters.contains(entity) {
//                     commands.entity(entity).trigger(DeactivateFluidFilter);
//                 }
//             }
//             button.0 = false;
//         }
//     }
// }
