use crate::prelude::*;
use bevy::{asset::Asset, prelude::*};
use bevy_seedling::sample::Sample;

mod ron;
pub mod textures;
mod tracking;

pub use ron::*;
pub use tracking::*;

pub fn plugin(app: &mut App) {
    // start asset loading
    app.add_plugins(tracking::plugin)
        .add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]))
        .load_resource_from_path::<Config>("config.ron")
        .load_resource::<AudioSources>()
        .load_resource::<Textures>()
        .add_plugins(TexturesLoaderPlugin);
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
    pub bevy: Handle<Image>,
    #[dependency]
    pub teabox: Handle<Image>,
    #[dependency]
    pub cup: Handle<Image>,
    #[dependency]
    pub tealeaf: Handle<Image>,
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bevy: assets.load("textures/bevy.png"),
            teabox: assets.load("textures/glass.png"),
            cup: assets.load("textures/glass.png"),
            tealeaf: assets.load("textures/glass.png"),
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
    #[dependency]
    pub cup_drop: Handle<Sample>,
    #[dependency]
    pub cup_drop_brewed: Handle<Sample>,

    // music
    #[dependency]
    pub bg_music: Handle<Sample>,
}

impl AudioSources {
    pub const CUP_DROP: &'static str = "audio/sfx/plop-reverse.ogg";
    pub const CUP_DROP_BREWED: &'static str = "audio/sfx/plop.ogg";
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
            cup_drop: assets.load(Self::CUP_DROP),
            cup_drop_brewed: assets.load(Self::CUP_DROP_BREWED),
        }
    }
}
