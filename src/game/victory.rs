use crate::{game::tea::TeaCounter, prelude::*};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        check_tea_counters.run_if(resource_exists::<Config>.and(resource_exists::<LoadedLevel>)),
    );
}

fn check_tea_counters(
    counters: Query<&TeaCounter>,
    config: Res<Config>,
    mut loaded_level: ResMut<LoadedLevel>,
    level_list: Res<LevelList>,
    mut screen: ResMut<NextState<Screen>>,
) {
    for counter in &counters {
        if counter.0 >= config.tea_particles_for_victory {
            if let Some(idx) = level_list.0.iter().position(|l| l == &loaded_level.0) {
                let new_idx = idx + 1;
                if new_idx < level_list.0.len() {
                    loaded_level.0 = level_list.0[new_idx].clone();
                } else {
                    screen.set(Screen::GameOver);
                }
            }
            break;
        }
    }
}
