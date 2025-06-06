use crate::prelude::*;
use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
/// player currency for buying tea-moving components
pub struct AvailableZenPoints(u32);

pub struct AvailableZenPointsPlugin;

impl Plugin for AvailableZenPointsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AvailableZenPoints>().add_systems(
            Update,
            set_on_level_start.run_if(resource_exists_and_changed::<LoadedLevel>),
        );

        //DEBUG
        app.add_systems(
            Update,
            log_change_in_available_zen_points.run_if(resource_changed::<AvailableZenPoints>),
        );
    }
}

fn set_on_level_start(
    game_level: Res<LoadedLevel>,
    mut available_zen_points: ResMut<AvailableZenPoints>,
    level_configs: Res<Assets<LevelConfig>>,
) {
    if let Some(config) = level_configs.get(&game_level.0) {
        available_zen_points.0 = config.zen_points;
    }
}

impl AvailableZenPoints {
    pub fn buy_if_affordable(&mut self, cost: u32) -> ActionPerformed {
        let affordable = self.0 >= cost;
        if affordable {
            self.0 -= cost;
        }
        ActionPerformed(affordable)
    }

    pub fn refund(&mut self, cost: u32) {
        self.0 += cost;
    }
}

fn log_change_in_available_zen_points(available_zen_points: Res<AvailableZenPoints>) {
    info!("There are {} available zen points.", available_zen_points.0);
}
