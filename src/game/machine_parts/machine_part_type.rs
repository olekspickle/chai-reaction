use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::PlacementContext;

#[derive(Component, Debug, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
pub struct MachinePartType {
    pub name: String,
    pub context: PlacementContext,
}
