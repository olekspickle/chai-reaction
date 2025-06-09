//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{
    game::input_dispatch::*, game::physics::PhysicsState, prelude::*, screens::gameplay::PauseLabel,
};
#[cfg(feature = "dev_native")]
use avian2d::prelude::*;
#[cfg(feature = "dev_native")]
use bevy::{dev_tools::states::log_transitions, prelude::*, ui::UiDebugOptions};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(toggle_pause);

    #[cfg(feature = "dev_native")]
    {
        app.add_plugins(PhysicsDebugPlugin::default());
        app.insert_gizmo_config(PhysicsGizmos::none(), GizmoConfig::default());
        app.add_systems(Update, log_transitions::<Screen>);
        app.add_observer(toggle_debug_ui);
    }
}

#[cfg(feature = "dev_native")]
fn toggle_debug_ui(
    _: Trigger<OnDebugUiToggle>,
    mut options: ResMut<UiDebugOptions>,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
) {
    options.toggle();
    let (_, config) = gizmo_config_store.config_mut::<PhysicsGizmos>();
    if options.enabled {
        *config = PhysicsGizmos::all();
    } else {
        *config = PhysicsGizmos::none();
    }
}

fn toggle_pause(
    _: Trigger<OnPauseToggle>,
    mut label: Query<(&mut BackgroundColor, &mut TextColor), With<PauseLabel>>,
    physics_state: ResMut<State<PhysicsState>>,
    mut next: ResMut<NextState<PhysicsState>>,
) {
    if let Ok((mut bg, mut color)) = label.single_mut() {
        match physics_state.get() {
            PhysicsState::Paused => {
                next.set(PhysicsState::Running);
                *color = TextColor(WHITEISH);
                *bg = BackgroundColor(TRANSPARENT);
                info!("paused: false");
            }
            PhysicsState::Running => {
                next.set(PhysicsState::Paused);
                *color = TextColor(GRAY);
                *bg = BackgroundColor(WHITEISH);
                info!("paused: true");
            }
        }
    }
}
