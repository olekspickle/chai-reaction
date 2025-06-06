use crate::{
    game::{
        heat::HeatSource,
        tea::{Tea, TeaCounter},
    },
    prelude::*,
};
use avian2d::{
    parry::shape::{Compound, SharedShape},
    prelude::*,
};
use bevy::{prelude::*, scene::ron::de};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct MachinePartConfig {
    pub cost: u32,
    pub is_dynamic: bool,
    #[serde(default)]
    pub texture_info: TextureInfo,
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
        colliders: Vec<Vec<Compound>>,
    },
    Sprite {
        #[serde(default)]
        offset: Vec2,
        #[serde(default)]
        layer: MachinePartLayer,
        #[serde(skip)]
        sprite: MachineSpriteInfo,
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
    HeatSource {
        #[serde(default)]
        offset: Vec2,
        radius: f32,
    },
    Tea {
        #[serde(default)]
        offset: Vec2,
        radius: f32,
    },
    VictorySensor {
        #[serde(default)]
        offset: Vec2,
        radius: f32,
    },
    FlowField {
        #[serde(default)]
        flow_texture_path: String,
        #[serde(skip)]
        flow_texture: MachineSpriteInfo,
        #[serde(skip)]
        #[reflect(ignore)]
        collider: Collider,
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct TextureInfo {
    #[serde(default)]
    pub frames: u32,
    #[serde(default)]
    pub rotations: u32,
    #[serde(default)]
    pub flippable: bool, //if flippable the 2nd half of the rotations will only be accessible while flipped
}

impl Default for TextureInfo {
    fn default() -> Self {
        Self {
            frames: 1,
            rotations: 1,
            flippable: false,
        }
    }
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct MachineSpriteInfo {
    pub image: Handle<Image>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
}




#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    Reflect,
    Serialize,
    Deserialize,
)]
pub struct PlacementContext {
    pub position: Vec3,
    pub rotation_index: u32,
    pub flipped: bool,
}


#[derive(Component)]
pub struct SpawnedMachinePart;

impl MachinePartConfig {
    pub fn spawn_sprites(&self, rotation_index: u32, mut commands: EntityCommands) {
        commands.with_children(|parent| {
            for subassembly in &self.subassemblies {
                if let SubAssembly::Sprite {
                    offset,
                    layer,
                    sprite,
                    ..
                } = subassembly
                {
                    let mut child = parent.spawn(Transform::from_xyz(offset.x, offset.y, layer.to_z()));

                    if let Some(layout) = &sprite.layout {
                        child.insert(Sprite {
                            image: sprite.image.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: layout.clone(),
                                index: rotation_index as usize,
                            }),
                            ..default()
                        });
                    } else {
                        child.insert(Sprite {
                            image: sprite.image.clone(),
                            ..default()
                        });
                    }
                }
            }
        });
    }

    pub fn spawn(&self, part_type: MachinePartType, commands: &mut Commands) {
        let context = part_type.context.clone();
        commands
            .spawn((
                SpawnedMachinePart,
                Transform::from_translation(context.position.clone()),
                part_type.clone(),
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
                            let mut child = parent.spawn(Transform::from_xyz(offset.x, offset.y, layer.to_z()));
                            child.insert(Pickable::default());

                            if let Some(layout) = &sprite.layout {
                                child.insert(Sprite {
                                    image: sprite.image.clone(),
                                    texture_atlas: Some(TextureAtlas {
                                        layout: layout.clone(),
                                        index: context.rotation_index as usize,
                                    }),
                                    ..default()
                                });
                            } else {
                                child.insert(Sprite {
                                    image: sprite.image.clone(),
                                    ..default()
                                });
                            }
                        }
                        SubAssembly::Collider {
                            offset, colliders, ..
                        } => {
                            // Select the set of colliders based on the current rotation index
                            if let Some(collider_set) = colliders.get(context.rotation_index as usize) {
                                for collider in collider_set {
                                    parent.spawn((
                                        Transform::from_xyz(offset.x, offset.y, 0.0),
                                        Collider::from(SharedShape::new(collider.clone())),
                                    ));
                                }
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
                        SubAssembly::HeatSource { offset, radius } => {
                            parent.spawn((
                                Transform::from_xyz(offset.x, offset.y, 0.0),
                                HeatSource,
                                Collider::circle(*radius),
                                Sensor,
                            ));
                        }
                        SubAssembly::Tea { offset, radius } => {
                            parent.spawn((
                                Transform::from_xyz(offset.x, offset.y, 0.0),
                                Tea,
                                Collider::circle(*radius),
                                Sensor,
                            ));
                        }
                        SubAssembly::VictorySensor { offset, radius } => {
                            parent.spawn((
                                Transform::from_xyz(offset.x, offset.y, 0.0),
                                TeaCounter::default(),
                                Collider::circle(*radius),
                                Sensor,
                            ));
                        }
                        SubAssembly::FlowField { flow_texture, collider, .. } => { 
                            parent.spawn((
                                FlowField { 
                                    sprite_info: flow_texture.clone(), 
                                    rotation_index: context.rotation_index as u32,
                                },
                                collider.clone(),
                                match &flow_texture.layout {
                                    Some(layout) => Sprite {
                                        image: flow_texture.image.clone(),
                                        color: Color::WHITE.with_alpha(0.3),
                                        texture_atlas: Some(TextureAtlas {
                                            layout: layout.clone(),
                                            index: context.rotation_index as usize,
                                        }),
                                        ..default()
                                    },
                                    None => Sprite {
                                        image: flow_texture.image.clone(),
                                        color: Color::WHITE.with_alpha(0.3),
                                        ..default()
                                    }
                                },
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
            if let Some(part_config) = machine_part_config_by_type.0.get(&ty.name) {
                available_zen_points.refund(part_config.cost);
                commands.entity(trigger.target()).despawn();
            }
        }
    }
}
