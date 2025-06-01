use bevy::{prelude::*, ui::Val::*};

mod interaction;
mod opts;
mod palette;
mod widget;

pub use interaction::*;
pub use opts::*;
pub use palette::*;
pub use widget::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        // https://github.com/IyesGames/iyes_perf_ui/issues/30
        // bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        bevy::render::diagnostic::RenderDiagnosticsPlugin,
        interaction::plugin,
    ));
}
