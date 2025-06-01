use crate::prelude::*;
use bevy::{asset::Asset, prelude::*};
use bevy_seedling::sample::Sample;

mod ron;
mod tracking;

pub use ron::*;
pub use tracking::*;

pub fn plugin(app: &mut App) {
    // start asset loading
    app.add_plugins(tracking::plugin)
        .add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]))
        .load_resource_from_path::<Config>("config.ron")
        .load_resource::<AudioSources>()
        .load_resource::<Textures>();
}

#[derive(Asset, Clone, Reflect, Resource)]
#[reflect(Resource)]
pub struct Fonts {
    #[dependency]
    pub fira: Handle<Font>,
}

#[derive(Asset, Clone, Reflect, Resource)]
#[reflect(Resource)]
pub struct Textures {
    #[dependency]
    pub github: Handle<Image>,
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            github: assets.load("textures/github.png"),
        }
    }
}

#[derive(Asset, Clone, Reflect, Resource)]
#[reflect(Resource)]
pub struct AudioSources {
    // SFX
    #[dependency]
    pub btn_hover: Handle<Sample>,
    #[dependency]
    pub btn_press: Handle<Sample>,

    // music
    #[dependency]
    pub bg_music: Handle<Sample>,
}

impl AudioSources {
    pub const BTN_HOVER: &'static str = "audio/sfx/btn-hover.ogg";
    pub const BTN_PRESS: &'static str = "audio/sfx/btn-press.ogg";

    pub const BG_MUSIC: &'static str = "audio/music/smnbl-time-for-fun.ogg";
}

impl FromWorld for AudioSources {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            btn_hover: assets.load(Self::BTN_HOVER),
            btn_press: assets.load(Self::BTN_PRESS),
            bg_music: assets.load(Self::BG_MUSIC),
        }
    }
}
