use crate::{
    game::{
        heat::HeatSource,
        machine_parts::{
            animator::{BasicSpriteAnimationController, SpriteFrames},
            particle_vessel::ParticleVessel,
        },
        tea::{Recipe, Tea, TeaSensor},
        tea_particles::TeaParticleVessel, ParticleLayer,
    },
    prelude::*,
};
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
    pub icon: PartIcon,
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
    RedBall,
    Collider {
        #[serde(default)]
        offset: Vec2,
        mesh_image_path: String,
        #[serde(skip)]
        #[reflect(ignore)]
        colliders: Vec<Vec<Compound>>,
    },
    ConveyorBelt {
        #[serde(default)]
        offset: Vec2,
        mesh_image_path: String,
        #[serde(skip)]
        #[reflect(ignore)]
        colliders: Vec<Vec<Compound>>,
        speed: f32,
    },
    CircleCollider {
        #[serde(default)]
        offset: Vec2,
        radius: f32,
    },
    FluidFilter {
        #[serde(default)]
        offset: Vec2,
        mesh_image_path: String,
        #[serde(skip)]
        #[reflect(ignore)]
        colliders: Vec<Vec<Compound>>,
    },
    FluidFilterButton {
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
    ParticleEmitter {
        #[serde(default)]
        offset: Vec2,
        spawn_rate: f32,
        initial_speed_min: f32,
        initial_speed_max: f32,
        initial_angle_deg_min: f32,
        initial_angle_deg_max: f32,
        particle_lifetime_s: f32,
        particle_gravity_scale: f32,
        #[serde(default)]
        kind: ParticleContents,
    },
    ParticleVessel {
        #[serde(default)]
        texture_path: String,
        #[serde(skip)]
        image: Handle<Image>,
        #[serde(default)]
        offset: Vec2,
        particle_lifetime_s: f32,
        particle_gravity_scale: f32,
        particle_radius: f32,
        #[serde(default)]
        kind: ParticleContents,
    },
    TeaParticleVessel {
        #[serde(default)]
        texture_path: String,
        #[serde(skip)]
        image: Handle<Image>,
        #[serde(default)]
        particle_texture_path: String,
        #[serde(skip)]
        particle_image: Handle<Image>,
        #[serde(default)]
        offset: Vec2,
        particle_gravity_scale: f32,
        particle_radius: f32,
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
    TeaSensor {
        #[serde(default)]
        offset: Vec2,
        mesh_image_path: String,
        #[serde(skip)]
        #[reflect(ignore)]
        colliders: Vec<Vec<Compound>>,
        #[serde(default)]
        recipe: Recipe,
        name: String,
        #[serde(skip)]
        icon: Handle<Image>,
        #[serde(default)]
        icon_asset_path: String,
    },
    FlowField {
        #[serde(default)]
        flow_texture_path: String,
        #[serde(skip)]
        flow_texture: MachineSpriteInfo,
        #[serde(skip)]
        #[reflect(ignore)]
        collider: Collider,
        flow_type: FlowType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Default)]
pub struct PartIcon {
    pub path: String,
    #[serde(skip)]
    pub handle: Option<Handle<Image>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct TextureInfo {
    #[serde(default)]
    pub frames: SpriteFrames,
    #[serde(default)]
    pub rotations: u32,
    #[serde(default)]
    pub flippable: bool, //if flippable the 2nd half of the rotations will only be accessible while flipped
}

impl Default for TextureInfo {
    fn default() -> Self {
        Self {
            frames: SpriteFrames::One,
            rotations: 1,
            flippable: false,
        }
    }
}

#[derive(Component)]
pub struct MachineSprite;

#[derive(Debug, Clone, Reflect, Default)]
pub struct MachineSpriteInfo {
    pub image: Handle<Image>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
pub struct PlacementContext {
    pub position: Vec3,
    pub rotation_index: u32,
    pub flipped: bool,
}

#[derive(Component)]
pub struct SpawnedMachinePart;

#[derive(Component)]
pub struct RedBall;

impl MachinePartConfig {
    pub fn spawn_sprites(&self, sprite_index: u32, mut commands: EntityCommands) {
        commands.with_children(|parent| {
            for subassembly in &self.subassemblies {
                if let SubAssembly::Sprite {
                    offset,
                    layer,
                    sprite,
                    ..
                } = subassembly
                {
                    let mut child =
                        parent.spawn(Transform::from_xyz(offset.x, offset.y, layer.to_z()));

                    if let Some(layout) = &sprite.layout {
                        child.insert(Sprite {
                            image: sprite.image.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: layout.clone(),
                                index: sprite_index as usize,
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

    pub fn spawn(
        &self,
        part_type: MachinePartType,
        commands: &mut Commands,
        sounds: &Res<AudioSources>,
        settings: &Res<Settings>,
        #[cfg(debug_assertions)] meshes: &mut ResMut<Assets<Mesh>>,
        #[cfg(debug_assertions)] materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Entity {
        let context = part_type.context.clone();
        let sprite_index = self.texture_info.frames.frames() * context.rotation_index;

        let mut part = commands.spawn((
            SpawnedMachinePart,
            LevelObject,
            Transform::from_translation(context.position),
            part_type.clone(),
            if self.is_dynamic {
                RigidBody::Dynamic
            } else {
                RigidBody::Static
            },
            Pickable::default(),
        ));

        match self.texture_info.frames {
            SpriteFrames::One => {}
            SpriteFrames::Basic(count, time) => {
                part.insert(BasicSpriteAnimationController {
                    frame_count: count,
                    current_frame: 0,
                    timer: Timer::from_seconds(time, TimerMode::Repeating),
                });
            }
        }
        part.observe(handle_erase_click);
        if self
            .subassemblies
            .iter()
            .any(|s| matches!(s, SubAssembly::RedBall))
        {
            part.insert(RedBall);
        }
        part.with_children(|parent| {
            for subassembly in &self.subassemblies {
                match subassembly {
                    SubAssembly::RedBall => {}
                    SubAssembly::Sprite {
                        offset,
                        layer,
                        sprite,
                        ..
                    } => {
                        let mut child =
                            parent.spawn(Transform::from_xyz(offset.x, offset.y, layer.to_z()));
                        child.insert(Pickable::default());
                        child.insert(MachineSprite);

                        if let Some(layout) = &sprite.layout {
                            child.insert(Sprite {
                                image: sprite.image.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: layout.clone(),
                                    index: sprite_index as usize,
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
                    SubAssembly::CircleCollider { offset, radius } => {
                        parent.spawn((
                            Transform::from_xyz(offset.x, offset.y, 0.0),
                            Collider::circle(*radius),
                        ));
                    }
                    SubAssembly::ConveyorBelt {
                        offset,
                        colliders,
                        speed,
                        ..
                    } => {
                        // Select the set of colliders based on the current rotation index
                        if let Some(collider_set) = colliders.get(context.rotation_index as usize) {
                            let actual_speed = if context.flipped { -*speed } else { *speed };
                            for collider in collider_set {
                                parent.spawn((
                                    Transform::from_xyz(offset.x, offset.y, 0.0),
                                    Collider::from(SharedShape::new(collider.clone())),
                                    crate::game::conveyor_belts::ConveyorBelt { speed: actual_speed },
                                ));
                            }
                        }
                    }
                    SubAssembly::FluidFilter {
                        offset, colliders, ..
                    } => {
                        // Select the set of colliders based on the current rotation index
                        if let Some(collider_set) = colliders.get(context.rotation_index as usize) {
                            for collider in collider_set {
                                parent.spawn((
                                    Transform::from_xyz(offset.x, offset.y, 0.0),
                                    Collider::from(SharedShape::new(collider.clone())),
                                    FluidFilter,
                                    CollisionLayers::new(
                                        ParticleLayer::Default,
                                        [ParticleLayer::Default, ParticleLayer::TeaLeaves],
                                    )
                                ));
                            }
                        }
                    }
                    SubAssembly::FluidFilterButton {
                        offset, colliders, ..
                    } => {
                        // Select the set of colliders based on the current rotation index
                        if let Some(collider_set) = colliders.get(context.rotation_index as usize) {
                            for collider in collider_set {
                                parent.spawn((
                                    Transform::from_xyz(offset.x, offset.y, 0.0),
                                    Collider::from(SharedShape::new(collider.clone())),
                                    FluidFilterButton::default(),
                                    Sensor,
                                ));
                            }
                        }
                    }
                    SubAssembly::ParticleEmitter {
                        offset,
                        spawn_rate,
                        initial_speed_min,
                        initial_speed_max,
                        initial_angle_deg_min,
                        initial_angle_deg_max,
                        particle_lifetime_s,
                        particle_gravity_scale,
                        kind,
                    } => {
                        parent.spawn((
                            Transform::from_xyz(offset.x, offset.y, 0.0),
                            ParticleEmitter::new(
                                *kind,
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
                    SubAssembly::ParticleVessel {
                        offset,
                        image,
                        particle_lifetime_s,
                        particle_gravity_scale,
                        kind,
                        particle_radius,
                        ..
                    } => {
                        parent.spawn((
                            Transform::from_xyz(offset.x, offset.y, 0.0),
                            ParticleVessel {
                                image: image.clone(),
                                completed: false,
                                kind: *kind,
                                particle_gravity_scale: *particle_gravity_scale,
                                particle_lifetime_s: *particle_lifetime_s,
                                particle_radius: *particle_radius,
                            },
                        ));
                    }
                    SubAssembly::TeaParticleVessel {
                        offset,
                        image,
                        particle_gravity_scale,
                        particle_image,
                        particle_radius,
                        ..
                    } => {
                        parent.spawn((
                            Transform::from_xyz(offset.x, offset.y, 0.0),
                            TeaParticleVessel {
                                image: image.clone(),
                                particle_image: particle_image.clone(),
                                completed: false,
                                particle_gravity_scale: *particle_gravity_scale,
                                particle_radius: *particle_radius,
                            },
                        ));
                    }
                    SubAssembly::HeatSource { offset, radius } => {
                        parent.spawn((
                            sfx_looping(sounds.stove_looping.clone(), settings.sfx()),
                            Transform::from_xyz(offset.x, offset.y, 0.0),
                            HeatSource,
                            Collider::circle(*radius),
                            Sensor,
                            #[cfg(debug_assertions)]
                            Mesh2d(meshes.add(Circle::new(*radius))),
                            #[cfg(debug_assertions)]
                            MeshMaterial2d(materials.add(Color::srgba(0.9, 0.7, 0.2, 0.01))),
                            Pickable::IGNORE,
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
                    SubAssembly::TeaSensor {
                        offset,
                        colliders,
                        recipe,
                        name,
                        icon,
                        ..
                    } => {
                        if let Some(collider_set) = colliders.get(context.rotation_index as usize) {
                            assert!(collider_set.len() == 1);
                            for collider in collider_set {
                                parent.spawn((
                                    Name::new(name.clone()),
                                    Transform::from_xyz(offset.x, offset.y, 0.0),
                                    Collider::from(SharedShape::new(collider.clone())),
                                    Sensor,
                                    Pickable::IGNORE,
                                    TeaSensor(*recipe, icon.clone()),
                                ));
                            }
                        }
                    }
                    SubAssembly::FlowField {
                        flow_texture,
                        collider,
                        flow_type,
                        ..
                    } => {
                        parent.spawn((
                            FlowField {
                                sprite_info: flow_texture.clone(),
                                rotation_index: context.rotation_index,
                                flow_type: *flow_type,
                            },
                            collider.clone(),
                            // match &flow_texture.layout {
                            //     Some(layout) => Sprite {
                            //         image: flow_texture.image.clone(),
                            //         color: Color::WHITE.with_alpha(0.3),
                            //         texture_atlas: Some(TextureAtlas {
                            //             layout: layout.clone(),
                            //             index: context.rotation_index as usize,
                            //         }),
                            //         ..default()
                            //     },
                            //     None => Sprite {
                            //         image: flow_texture.image.clone(),
                            //         color: Color::WHITE.with_alpha(0.3),
                            //         ..default()
                            //     },
                            // },
                        ));
                    }
                }
            }
        });

        part.id()
    }
}

fn handle_erase_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    picking_state: Res<PickingState>,
    part_type: Query<&MachinePartType>,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
    mut available_zen_points: ResMut<AvailableZenPoints>,
    initial_part: Query<&IsInitialPart>,
    sounds: Res<AudioSources>,
    settings: Res<Settings>,
    editor_mode: Res<EditorMode>,
) {
    if *picking_state == PickingState::Erasing {
        if editor_mode.0 || !initial_part.contains(trigger.target()) {
            if let Ok(ty) = part_type.get(trigger.target()) {
                if let Some(part_config) = machine_part_config_by_type.0.get(&ty.name) {
                    let source = sounds.cancel_piece.clone();
                    commands.spawn(sfx(source, settings.sfx()));

                    available_zen_points.refund(part_config.cost);
                    commands.entity(trigger.target()).despawn();
                }
            }
        }
    }
}
