use bevy::{prelude::*, ui::Val::*};
pub use interaction::*;
pub use opts::*;
pub use palette::*;
pub use widget::*;
use crate::prelude::InGameUiPlugin;

mod interaction;
mod opts;
mod palette;
mod widget;
pub mod in_game_ui;
pub mod tags;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        // https://github.com/IyesGames/iyes_perf_ui/issues/30
        // bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        bevy::render::diagnostic::RenderDiagnosticsPlugin,
        interaction::plugin,
    ))
        .add_plugins(InGameUiPlugin);
}
