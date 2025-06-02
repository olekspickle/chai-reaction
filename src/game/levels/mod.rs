use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

mod sink;

pub fn plugin(app: &mut App) {
    app.insert_state(GameLevel::Sink)
        .add_systems(OnEnter(Screen::Gameplay), set_game_level_to_sink)
        .add_systems(
            OnEnter(GameLevel::Sink),
            sink::spawn_scene.run_if(resource_exists::<Config>),
        );
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameLevel {
    Sink,
    GreatGap,
    TimeWindow,
}

#[derive(Component)]
pub struct WaterBucket;
#[derive(Component)]
pub struct Stove;
#[derive(Component)]
pub struct TeaBox;

fn set_game_level_to_sink(cfg: Res<Config>, mut game_level: ResMut<NextState<GameLevel>>) {
    info!("config: {cfg:?}");
    game_level.set(GameLevel::Sink);
}
