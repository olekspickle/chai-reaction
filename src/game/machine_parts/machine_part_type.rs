use bevy::prelude::*;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::prelude::PlacementContext;

#[derive(
    Component,
    Debug,
    Clone,
    PartialEq,
    Default,
    Reflect,
    Serialize,
    Deserialize,
)]
pub struct MachinePartType {
    pub name: String,
    pub context: PlacementContext,
}

