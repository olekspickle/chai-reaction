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
    #[dependency]
    pub exit: Handle<Image>,
    #[dependency]
    pub pause: Handle<Image>,
    #[dependency]
    pub play: Handle<Image>,
    #[dependency]
    pub reset: Handle<Image>,
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bevy: assets.load("textures/bevy.png"),
            teabox: assets.load("textures/glass.png"),
            cup: assets.load("textures/glass.png"),
            tealeaf: assets.load("textures/glass.png"),
            exit: assets.load("textures/Icons/exit.png"),
            pause: assets.load("textures/Icons/exit.png"),
            play: assets.load("textures/Icons/play.png"),
            reset: assets.load("textures/Icons/reset.png"),
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
    #[dependency]
    pub cancel_piece: Handle<Sample>,
    #[dependency]
    pub place_piece: Handle<Sample>,
    #[dependency]
    pub pipe_hits: Vec<Handle<Sample>>,
    #[dependency]
    pub into_cup_plops: Vec<Handle<Sample>>,
    #[dependency]
    pub stove_looping: Handle<Sample>,

    // music
    #[dependency]
    pub bg_music: Handle<Sample>,
    #[dependency]
    pub menu: Handle<Sample>,
}

impl AudioSources {
    pub const CUP_DROP: &'static str = "audio/sfx/plop-reverse.ogg";
    pub const CUP_DROP_BREWED: &'static str = "audio/sfx/plop.ogg";
    pub const BTN_HOVER: &'static str = "audio/sfx/btn-hover.ogg";
    pub const BTN_PRESS: &'static str = "audio/sfx/btn-press.ogg";
    pub const CANCEL_PIECE: &'static str = "audio/sfx/cancelPiece.ogg";
    pub const PLACE_PIECE: &'static str = "audio/sfx/placePiece.ogg";
    pub const PIPE_HITS: &'static [&'static str] = &[
        "audio/sfx/pipeHit1.ogg",
        "audio/sfx/pipeHit2.ogg",
        "audio/sfx/pipeHit3.ogg",
        "audio/sfx/pipeHit4.ogg",
    ];
    pub const INTO_CUP_PLOPS: &'static [&'static str] =
        &["audio/sfx/intoTheCup1.ogg", "audio/sfx/intoTheCup2.ogg"];
    pub const STOVE_LOOPING: &'static str = "audio/sfx/stoveLoopingBitcrushed.ogg";

    pub const BG_MUSIC: &'static str = "audio/music/lofi-tea.ogg";
    pub const MENU: &'static str = "audio/music/how-to-make-tea.ogg";
}

impl FromWorld for AudioSources {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            menu: assets.load(Self::MENU),
            bg_music: assets.load(Self::BG_MUSIC),
            btn_hover: assets.load(Self::BTN_HOVER),
            btn_press: assets.load(Self::BTN_PRESS),
            cup_drop: assets.load(Self::CUP_DROP),
            cup_drop_brewed: assets.load(Self::CUP_DROP_BREWED),
            cancel_piece: assets.load(Self::CANCEL_PIECE),
            place_piece: assets.load(Self::PLACE_PIECE),
            pipe_hits: Self::PIPE_HITS.iter().map(|p| assets.load(*p)).collect(),
            into_cup_plops: Self::INTO_CUP_PLOPS
                .iter()
                .map(|p| assets.load(*p))
                .collect(),
            stove_looping: assets.load(Self::STOVE_LOOPING),
        }
    }
}
