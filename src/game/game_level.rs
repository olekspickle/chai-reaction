use bevy::prelude::*;

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
        app.insert_state(GameLevel::TheSink);
    }
}