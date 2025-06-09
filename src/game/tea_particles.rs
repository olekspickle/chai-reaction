use bevy::prelude::*;
use avian2d::prelude::*;
use crate::{
    prelude::*,
    game::tea::Tea,
    game::ParticleLayer,
    prelude::Config,
    screens::Screen,
    game::machine_parts::particle_vessel::scan_image_for_circles,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        do_particle_vessels.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct TeaParticle;

#[derive(Component, Debug, Clone)]
pub struct TeaParticleVessel {
    pub image: Handle<Image>,
    pub particle_image: Handle<Image>,
    pub completed: bool,
    pub particle_gravity_scale: f32, // How much Avian's gravity affects the particle
    pub particle_radius: f32,
}

pub fn do_particle_vessels(
    cfg: Res<Config>,
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut vessels: Query<(&mut TeaParticleVessel, &GlobalTransform)>,
) {
    for (mut vessel, global_transform) in vessels.iter_mut() {
        if vessel.completed {
            continue;
        }

        vessel.completed = true;

        let Some(img) = images.get(&vessel.image) else {
            continue;
        };

        let relative_positions = scan_image_for_circles(img, vessel.particle_radius as i32);
        // let relative_positions = vec![IVec2::new(0.0,m, y)]


        for position in relative_positions {
            let spawn_position = global_transform.translation().truncate() + position.as_vec2();


            commands.spawn((
                Sprite::from_image(vessel.particle_image.clone()),
                LevelObject,
                Transform::from_translation(spawn_position.extend(0.0)),
                // --- Avian2D Physics Components ---
                RigidBody::Dynamic,
                Collider::circle(vessel.particle_radius),
                GravityScale(vessel.particle_gravity_scale),
                Friction::new(cfg.physics.water.friction)
                    .with_combine_rule(CoefficientCombine::Multiply),
                Restitution::new(cfg.physics.water.restitution)
                    .with_combine_rule(CoefficientCombine::Min), // How bounciness is combined
                Mass(0.1),
                CollisionLayers::new(
                    ParticleLayer::TeaLeaves,
                    [
                        ParticleLayer::Default,
                        ParticleLayer::Fluid,
                        ParticleLayer::TeaLeaves,
                    ],
                ),
                SleepingDisabled,
                TeaParticle,
                Tea,
            ));
        }
    }
}
