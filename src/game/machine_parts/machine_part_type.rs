use bevy::prelude::*;
use enum_iterator::{Sequence};
use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Reflect, Serialize, Deserialize)]
pub struct MachinePartType(pub String);
