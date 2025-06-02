use bevy::prelude::*;
use crate::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameLevel{
    TheSink,
    TheGreatGap,
    TheTimeWindow,
    // AddOthers
}

pub struct GameLevelPlugin;

impl Plugin for GameLevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameLevel::TheSink)
            .add_systems(OnEnter(Screen::Gameplay), set_game_level_to_sink);
    }
}

fn set_game_level_to_sink(
    mut game_level: ResMut<NextState<GameLevel>>,
){
    game_level.set(GameLevel::TheSink);
}