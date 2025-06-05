use crate::prelude::*;
use avian2d::{
    parry::shape::{Compound, SharedShape},
    prelude::*,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct MachinePartConfig {
    pub cost: u32,
    pub is_dynamic: bool,
    #[serde(default)]
    pub subassemblies: Vec<SubAssembly>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum MachinePartLayer {
    #[default]
    Foreground,
    Background,
}

impl MachinePartLayer {
    pub fn to_z(&self) -> f32 {
        match self {
            MachinePartLayer::Foreground => 0.0,
            MachinePartLayer::Background => -100.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum SubAssembly {
    Collider {
        #[serde(default)]
        offset: Vec2,
        mesh_image_path: String,
        #[serde(skip)]
        #[reflect(ignore)]
        colliders: Vec<Compound>,
    },
    Sprite {
        #[serde(default)]
        offset: Vec2,
        #[serde(default)]
        layer: MachinePartLayer,
        #[serde(skip)]
        sprite: Handle<Image>,
        #[serde(default)]
        sprite_asset_path: String,
    },
    WaterEmitter {
        #[serde(default)]
        offset: Vec2,
        spawn_rate: f32,
        initial_speed_min: f32,
        initial_speed_max: f32,
        initial_angle_deg_min: f32,
        initial_angle_deg_max: f32,
        particle_lifetime_s: f32,
        particle_gravity_scale: f32,
    },
}

impl MachinePartConfig {
    pub fn spawn_sprites(&self, mut commands: EntityCommands) {
        commands.with_children(|parent| {
            for subassembly in &self.subassemblies {
                if let SubAssembly::Sprite {
                    offset,
                    layer,
                    sprite,
                    ..
                } = subassembly
                {
                    parent.spawn((
                        Transform::from_xyz(offset.x, offset.y, layer.to_z()),
                        Sprite {
                            image: sprite.clone(),
                            ..default()
                        },
                    ));
                }
            }
        });
    }

    pub fn spawn(&self, position: Vec3, part_type: MachinePartType, commands: &mut Commands) {
        commands
            .spawn((
                Transform::from_translation(position),
                part_type,
                if self.is_dynamic {
                    RigidBody::Dynamic
                } else {
                    RigidBody::Static
                },
                Pickable::default(),
            ))
            .observe(handle_erase_click)
            .with_children(|parent| {
                for subassembly in &self.subassemblies {
                    match subassembly {
                        SubAssembly::Sprite {
                            offset,
                            layer,
                            sprite,
                            ..
                        } => {
                            parent.spawn((
                                Transform::from_xyz(offset.x, offset.y, layer.to_z()),
                                Sprite {
                                    image: sprite.clone(),
                                    ..default()
                                },
                            ));
                        }
                        SubAssembly::Collider {
                            offset, colliders, ..
                        } => {
                            for collider in colliders {
                                parent.spawn((
                                    Transform::from_xyz(offset.x, offset.y, 0.0),
                                    Collider::from(SharedShape::new(collider.clone())),
                                ));
                            }
                        }
                        SubAssembly::WaterEmitter {
                            offset,
                            spawn_rate,
                            initial_speed_min,
                            initial_speed_max,
                            initial_angle_deg_min,
                            initial_angle_deg_max,
                            particle_lifetime_s,
                            particle_gravity_scale,
                        } => {
                            parent.spawn((
                                Transform::from_xyz(offset.x, offset.y, 0.0),
                                ParticleEmitter::new(
                                    ParticleKind::Water,
                                    *spawn_rate,
                                    *initial_speed_min,
                                    *initial_speed_max,
                                    *initial_angle_deg_min,
                                    *initial_angle_deg_max,
                                    *particle_lifetime_s,
                                    *particle_gravity_scale,
                                ),
                            ));
                        }
                    }
                }
            });
    }
}

fn handle_erase_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    picking_state: Res<PickingState>,
    part_type: Query<&MachinePartType>,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
    mut available_zen_points: ResMut<AvailableZenPoints>,
) {
    if *picking_state == PickingState::Erasing {
        if let Ok(ty) = part_type.get(trigger.target()) {
            if let Some(part_config) = machine_part_config_by_type.0.get(&ty.0) {
                available_zen_points.refund(part_config.cost);
                commands.entity(trigger.target()).despawn();
            }
        }
    }
}
