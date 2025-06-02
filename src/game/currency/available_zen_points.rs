use bevy::prelude::*;
use crate::prelude::*;

#[derive(Resource, Debug, Default)]
/// player currency for buying tea-moving components
pub struct AvailableZenPoints(pub u32);

pub struct AvailableZenPointsPlugin;

impl Plugin for AvailableZenPointsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AvailableZenPoints>()
            .add_systems(Update, set_on_level_start.run_if(state_changed::<GameLevel>));
    }
}

fn set_on_level_start(
    game_level: Res<State<GameLevel>>,
    mut available_zen_points: ResMut<AvailableZenPoints>,
    initial_zen_points: Res<InitialZenPointByLevel>
) {
    let game_level = game_level.get();
    if let Some(initial_points) = initial_zen_points.0.get(game_level) {
        available_zen_points.0 = *initial_points;
    }
}