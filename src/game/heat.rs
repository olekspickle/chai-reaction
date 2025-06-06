use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, apply_heat);
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct HeatSource;

fn apply_heat(
    collisions: Collisions,
    heat_sources: Query<Entity, With<HeatSource>>,
    mut particles: Query<(Entity, &mut Particle)>,
    time: Res<Time>,
) {
    for heat_source in &heat_sources {
        for (particle_entity, mut particle) in &mut particles {
            if collisions.contains(heat_source, particle_entity) {
                info!("heat source collided");
                particle.contents.heat += time.delta().as_secs_f32();
            }
        }
    }
}
