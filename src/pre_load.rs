use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
#[reflect(Resource)]
pub struct Config {
    pub sound: Sound,
    pub physics: Physics,
    pub credits: Credits,
    pub droplet_radius: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Physics {
    pub water: Water,
    pub gravity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Credits {
    pub assets: Vec<(String, String)>,
    pub devs: Vec<(String, String)>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Water {
    pub friction: f32,
    pub restitution: f32,
}
