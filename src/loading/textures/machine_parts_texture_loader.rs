use crate::prelude::*;
use bevy::prelude::*;

pub struct MachinePartsTextureLoaderPlugin;

impl Plugin for MachinePartsTextureLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_machine_parts_textures);
    }
}

fn load_machine_parts_textures(
    mut machine_part_config_by_type: ResMut<MachinePartConfigByType>,
    asset_server: Res<AssetServer>,
){
    for mut config in machine_part_config_by_type.0.values_mut(){
        let handle = asset_server.load::<Image>(config.sprite_asset_path.as_str());
        config.sprite = handle;
    }
}