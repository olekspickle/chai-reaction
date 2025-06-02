use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct MachinePartConfigByType(pub HashMap<MachinePartType, MachinePartConfig>);

pub struct MachinePartConfigByTypePlugin;

impl Plugin for MachinePartConfigByTypePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MachinePartConfigByType>();
    }
}

impl Default for MachinePartConfigByType {
    fn default() -> Self {
        Self(HashMap::from(
            [
                (MachinePartType::Scale, MachinePartConfig{
                    cost: 42,
                    sprite_asset_path: String::from("textures/bevy.png"),
                    ..default()
                }),
                (MachinePartType::Block, MachinePartConfig{
                    cost: 8,
                    sprite_asset_path: String::from("textures/bevy.png"),
                    ..default()
                })
            ]
        ))
    }
}