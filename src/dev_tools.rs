//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{game::input_dispatch::*, prelude::*, screens::gameplay::PauseLabel};
#[cfg(feature = "dev_native")]
use bevy::{dev_tools::states::log_transitions, prelude::*, ui::UiDebugOptions};
use avian2d::prelude::*;

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
fn toggle_debug_ui(_: Trigger<OnDebugUiToggle>, mut options: ResMut<UiDebugOptions>, mut gizmo_config_store: ResMut<GizmoConfigStore> ) {
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
    mut settings: ResMut<Settings>,
    mut time: ResMut<Time<Virtual>>,
    mut label: Query<(&mut BackgroundColor, &mut TextColor), With<PauseLabel>>,
) {
    if let Ok((mut bg, mut color)) = label.single_mut() {
        if time.is_paused() || settings.paused {
            time.unpause();
            *color = TextColor(WHITEISH);
            *bg = BackgroundColor(TRANSPARENT);
        } else {
            time.pause();
            *color = TextColor(GRAY);
            *bg = BackgroundColor(WHITEISH);
        }
    }

    settings.paused = !settings.paused;
}
