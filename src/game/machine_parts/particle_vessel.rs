use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{game::ParticleLayer, prelude::{Config, Particle, ParticleContents}, screens::Screen, ui::WATER};


pub struct ParticleVesselPlugin;
impl Plugin for ParticleVesselPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, do_particle_vessels.run_if(in_state(Screen::Gameplay)));
    }
}


#[derive(Component, Debug, Clone)]
pub struct ParticleVessel {
    pub image: Handle<Image>,
    pub completed: bool,
    pub kind: ParticleContents,
    pub particle_gravity_scale: f32, // How much Avian's gravity affects the particle
    pub particle_lifetime_s: f32,
}

pub fn do_particle_vessels (
    cfg: Res<Config>,
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut vessels: Query<(&mut ParticleVessel, &GlobalTransform)>
) {
    for (mut vessel, global_transform) in vessels.iter_mut() {
        if vessel.completed {
            continue;
        }
        
        vessel.completed = true;

        let Some(img) = images.get(&vessel.image) else {
            continue;
        };

        let relative_positions = scan_image_for_circles(img, cfg.droplet_radius as i32);
        // let relative_positions = vec![IVec2::new(0.0,m, y)]
    
        let mesh = meshes.add(Circle::new(cfg.droplet_radius));

        for position in relative_positions {
            let spawn_position = global_transform.translation().truncate() + position.as_vec2();

            let contents = vessel.kind.clone();
            let material = materials.add(WATER);
            
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material),
                Transform::from_translation(spawn_position.extend(0.0)),
                // --- Avian2D Physics Components ---
                RigidBody::Dynamic,
                Collider::circle(cfg.droplet_radius),
                GravityScale(vessel.particle_gravity_scale),
                Friction::new(cfg.physics.water.friction)
                    .with_combine_rule(CoefficientCombine::Multiply),
                Restitution::new(cfg.physics.water.restitution)
                    .with_combine_rule(CoefficientCombine::Min), // How bounciness is combined
                Mass(0.1),
                CollisionLayers::new(
                    ParticleLayer::Fluid,
                    [ParticleLayer::Default, ParticleLayer::Fluid, ParticleLayer::TeaLeaves],
                ),
                SleepingDisabled,
                Particle {
                    lifetime: Timer::from_seconds(vessel.particle_lifetime_s, TimerMode::Once),
                    contents,
                },

            ));
        }
    
    }

}

pub fn scan_image_for_circles(image: &Image, radius: i32) -> Vec<IVec2> {
    let (width, height) = image.size().as_ivec2().into();
    let mut valid_spawns = Vec::new();

    let r_squared = radius * radius;


    let mut pixel_data: Vec<bool> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let color = image.get_color_at(x as u32, y as u32);
            let is_transparent = color.is_ok_and(|c| c.is_fully_transparent());
            pixel_data.push(!is_transparent);
        }
    }

    // println!("Pixel data:");
    // for y in 0..height {
    //     for x in 0..width {
    //         let idx = (y * width + x) as usize;
    //         if pixel_data[idx] {
    //             print!("██");
    //         } else {
    //             print!("  ");
    //         }
    //     }
    //     println!();
    // }

    //check pixels are all true
    for y in radius..(height as i32) {
        for x in radius..(width as i32) {
            let mut all_valid = true;

            let mut valid_positions = Vec::new();

            'outer: for dx in (0 - radius)..radius {
                for dy in (0 - radius)..radius {
                    let dist_sq = dx * dx + dy * dy;
                    if dist_sq > r_squared {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 || nx >= width || ny >= height {
                        all_valid = false;
                        break 'outer;
                    }
                    let idx = (ny * width + nx) as usize;
                    if !pixel_data[idx] {
                        all_valid = false;
                        break 'outer;
                    }
                    valid_positions.push(IVec2::new(nx, ny));
                }
            }
            if all_valid {
                for pos in &valid_positions {
                    let idx = (pos.y * width + pos.x) as usize;
                    pixel_data[idx] = false;
                }
                // Subtract half the width and height to center the spawn positions
                valid_spawns.push(IVec2::new(x - width / 2, y - height / 2));
            }
        }
    }

    // println!("Pixel data:");
    // for y in 0..height {
    //     for x in 0..width {
    //         let idx = (y * width + x) as usize;
    //         if pixel_data[idx] {
    //             print!("██");
    //         } else {
    //             print!("  ");
    //         }
    //     }
    //     println!();
    // }

    valid_spawns
}