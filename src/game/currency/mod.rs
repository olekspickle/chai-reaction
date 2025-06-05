use crate::prelude::*;
use bevy::prelude::*;

pub mod available_zen_points;
pub mod initial_zen_points_by_level;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AvailableZenPointsPlugin, InitialZenPointByLevelPlugin));
    }
}
