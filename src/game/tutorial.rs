use super::*;
use crate::loading::{LoadResource, RonAssetPlugin};
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    ui::Val::*,
};
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Step>();
    app.enable_state_scoped_entities::<Step>();

    app.add_plugins(RonAssetPlugin::<Tutorial>::new(&["tutorial.ron"]))
        .register_asset_loader(TutorialListLoader)
        .load_resource_from_path::<Tutorial>("tutorial.ron");

    app.add_systems(OnEnter(Screen::Gameplay), render_step)
        .add_systems(OnEnter(Screen::Title), reset_tutorial)
        .add_systems(
            Update,
            (next_step, render_step.run_if(state_changed::<Step>))
                .run_if(resource_exists::<Tutorial>.and(in_state(Screen::Gameplay))),
        );
}

#[derive(Default, Asset, Resource, Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct Tutorial(pub Vec<String>);

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
struct Step(usize);

#[derive(Component)]
pub struct TutorialModal;

fn reset_tutorial(mut settings: ResMut<Settings>, mut step: ResMut<NextState<Step>>) {
    settings.tutorial = true;
    step.set(Step(0));
}

fn render_step(
    step: Res<State<Step>>,
    mut commands: Commands,
    text: Res<Tutorial>,
    textures: Res<Textures>,
) {
    if step.0 > 6 {
        return;
    }

    let txt = text.0[step.0].clone();
    match step.0 {
        0 => {
            commands.spawn((
                StateScoped(step.clone()),
                tutorial_modal(txt),
                ImageNode::from(textures.chill.clone()),
            ));
        }
        1 => {
            commands.spawn((
                StateScoped(step.clone()),
                tutorial_modal(txt),
                ImageNode::from(textures.kidnap.clone()),
            ));
        }
        _ => {
            commands.spawn((StateScoped(step.clone()), tutorial_modal(txt)));
        }
    }
}

fn tutorial_modal(s: String) -> impl Bundle {
    (
        StateScoped(Screen::Gameplay),
        Name::new("Tutorial"),
        TutorialModal,
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Vh(5.0),
            ..default()
        },
        children![(
            Node {
                width: Percent(50.0),
                height: Percent(50.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba_u8(58, 68, 103, 240)),
            BorderColor(WHITEISH),
            children![label(s), label("\n[SPACE] to continue")]
        )],
    )
}

fn next_step(
    action: Query<&ActionState<Action>>,
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    current: ResMut<State<Step>>,
    mut next: ResMut<NextState<Step>>,
) {
    if let Ok(state) = action.single() {
        if state.just_pressed(&Action::TogglePhysics) && settings.tutorial {
            info!("current: {}", current.0);
            if current.0 > 6 {
                settings.tutorial = false;
                if !settings.modals.is_empty() {
                    commands.trigger(OnPopModal);
                }
            }

            next.set(Step(current.0 + 1));
        }
    }
}

#[derive(Default)]
struct TutorialListLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum TutorialListLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

impl AssetLoader for TutorialListLoader {
    type Asset = Tutorial;
    type Settings = ();
    type Error = TutorialListLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let steps = ron::de::from_bytes::<Vec<String>>(&bytes)?;

        Ok(Tutorial(steps))
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
