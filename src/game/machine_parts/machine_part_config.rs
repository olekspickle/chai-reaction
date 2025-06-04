use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use avian2d::parry::shape::Compound;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct MachinePartConfig {
    pub cost: u32,
    pub is_dynamic: bool,
    #[serde(skip)]
    pub sprite: Handle<Image>,
    pub sprite_asset_path: String,
    #[serde(skip)]
    pub background_sprite: Option<Handle<Image>>,
    #[serde(default)]
    pub background_sprite_asset_path: Option<String>,
    #[serde(default)]
    pub mesh_image_path: Option<String>,
    #[serde(skip)]
    #[reflect(ignore)]
    pub colliders: Vec<Compound>,
}
