use bevy::prelude::*;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Component,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Reflect,
    Serialize,
    Deserialize,
)]
pub struct MachinePartType(pub String);
