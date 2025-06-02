use bevy::prelude::*;
use enum_iterator::{Sequence};
use strum_macros::EnumIter;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Sequence, EnumIter, Default)]
pub enum MachinePartType{
    #[default]
    Scale,
    Block,
    // AddOthers
}