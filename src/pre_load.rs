use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
#[reflect(Resource)]
pub struct Config {
    pub sound: Sound,
    pub physics: Physics,
    pub credits: Credits,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Physics {
    pub gravity: f32,
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
    pub initial_density: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Credits {
    pub assets: Vec<(String, String)>,
    pub devs: Vec<(String, String)>,
}
