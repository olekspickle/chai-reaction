use bevy::prelude::*;
use enum_iterator::{Sequence};
use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Sequence, EnumIter, Default, Reflect, Serialize, Deserialize)]
pub enum MachinePartType{
    #[default]
    Scale,
    Block,
    // AddOthers
}
