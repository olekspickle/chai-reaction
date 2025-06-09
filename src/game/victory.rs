use crate::{
    game::tea::{Satisfied, TeaSensor},
    game::physics::PhysicsState,
    prelude::*,
    screens::gameplay::NextLevel,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        check_tea_counters.run_if(resource_exists::<Config>.and(resource_exists::<LoadedLevel>)),
    );
}

fn check_tea_counters(
    sensors: Query<Has<Satisfied>, With<TeaSensor>>,
    loaded_level: ResMut<LoadedLevel>,
    level_list: Res<LevelList>,
    mut physics_state: ResMut<NextState<PhysicsState>>,
    mut commands: Commands,
) {
    if !sensors.is_empty() && sensors.iter().all(|s| s) {
        if let Some(idx) = level_list.0.iter().position(|l| l == &loaded_level.0) {
            let new_idx = idx + 1;
            if new_idx < level_list.0.len() {
                // save next level id and spawn a modal in gameplay screen
                commands.insert_resource(NextLevel(new_idx));
                commands.trigger(OnNewModal(Modal::LevelFinished));
                physics_state.set(PhysicsState::Paused);
            }
        }
    }
}
