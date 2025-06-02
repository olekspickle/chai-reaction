use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct MachinePartConfig{
    pub cost: u32,
    pub sprite: Handle<Image>,
    pub sprite_asset_path: String,
}