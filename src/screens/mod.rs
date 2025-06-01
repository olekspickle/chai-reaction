//! The game's main screen states and transitions between them.

use crate::prelude::*;
use bevy::prelude::*;

mod credits;
mod gameover;
pub mod gameplay;
mod loading;
pub mod settings;
mod splash;
mod title;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        settings::plugin,
        credits::plugin,
        gameplay::plugin,
        gameover::plugin,
    ))
    .add_systems(Update, track_last_screen.run_if(state_changed::<Screen>))
    .add_observer(on_back)
    .add_observer(on_go_to);
}

/// The game's main screen states.
/// See <https://bevy-cheatbook.github.io/programming/states.html>
/// Or <https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs>
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Screen {
    // Bevy tribute <3
    #[default]
    Splash,
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    Credits,
    Settings,
    // Here the menu is drawn and waiting for player interaction
    Title,
    // During this State the actual game logic is executed
    Gameplay,
    GameOver,
}

// TODO: figure out how to make it a cool observer
// mut transitions: Trigger<StateTransitionEvent<Screen>>,
fn track_last_screen(
    mut transitions: EventReader<StateTransitionEvent<Screen>>,
    mut settings: ResMut<Settings>,
) {
    let Some(transition) = transitions.read().last() else {
        return;
    };
    settings.last_screen = transition.clone().exited.unwrap_or(Screen::Title);
}

fn on_back(
    trigger: Trigger<OnBack>,
    mut next_screen: ResMut<NextState<Screen>>,
    screen: Res<State<Screen>>,
) {
    // Do not go to the title on back, we'd rather handle it in gameplay observers
    if *screen.get() == Screen::Gameplay {
        return;
    }

    let back = trigger.event();
    next_screen.set(back.0.clone());
}

pub fn on_go_to(trig: Trigger<OnGoTo>, mut next_screen: ResMut<NextState<Screen>>) {
    let go_to = trig.event();
    next_screen.set(go_to.0.clone());
}

// TODO: figure out nice click_go_to(Screen::Title) HOF
// fn click_go_to<E, B, M>(s: Screen) -> impl IntoObserverSystem<OnPress, B, M> {
//     |_: Trigger<OnPress>, mut cmds: Commands| cmds.trigger(OnGoTo(s.clone()))
// }
pub mod to {
    use super::*;

    pub fn title(_: Trigger<OnPress>, mut cmds: Commands) {
        cmds.trigger(OnGoTo(Screen::Title));
    }
    pub fn settings(_: Trigger<OnPress>, mut cmds: Commands) {
        cmds.trigger(OnGoTo(Screen::Settings));
    }
    pub fn credits(_: Trigger<OnPress>, mut cmds: Commands) {
        cmds.trigger(OnGoTo(Screen::Credits));
    }
    pub fn gameplay_or_loading(
        _: Trigger<OnPress>,
        resource_handles: Res<ResourceHandles>,
        mut next_screen: ResMut<NextState<Screen>>,
    ) {
        if resource_handles.is_all_done() {
            next_screen.set(Screen::Gameplay);
        } else {
            next_screen.set(Screen::Loading);
        }
    }
}
