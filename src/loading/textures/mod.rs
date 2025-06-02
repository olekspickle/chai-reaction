use bevy::prelude::*;
use crate::prelude::*;

pub mod machine_parts_texture_loader;

pub struct TexturesLoaderPlugin;

impl Plugin for TexturesLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MachinePartsTextureLoaderPlugin);
    }
}