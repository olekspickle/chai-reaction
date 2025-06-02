use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MachinePartType{
    Scale,
    Block,
    AddOthers
}