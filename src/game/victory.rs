use crate::{game::tea::TeaCounter, prelude::*};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, check_tea_counters.run_if(resource_exists::<Config>));
}

fn check_tea_counters(counters: Query<&TeaCounter>, config: Res<Config>) {
    for counter in &counters {
        if counter.0 >= config.tea_particles_for_victory {
            println!("YOU WIN!");
        }
    }
}
