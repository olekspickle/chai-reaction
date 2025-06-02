use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct InitialZenPointByLevel(pub HashMap<GameLevel, u32>);

pub struct InitialZenPointByLevelPlugin;

impl Plugin for InitialZenPointByLevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InitialZenPointByLevel>();
    }
}

impl Default for InitialZenPointByLevel {
    fn default() -> Self {
        Self(HashMap::from(
            [
                (GameLevel::TheSink, 420),
                (GameLevel::TheGreatGap, 600)
            ]
        ))
    }
}