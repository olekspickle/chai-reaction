use crate::prelude::*;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub enum MachinePartRequest {
    SpawnMachinePart(MachinePartSpawnRequest),
    WhateverOtherRequests,
}

#[derive(Default, Debug)]
pub struct MachinePartSpawnRequest {
    pub part_type: MachinePartType,
    pub location: Vec3,
    pub initial_part: bool,
    pub free: bool,
    // pub location: PlacementContext,
}

pub struct MachinePartEventsPlugin;

impl Plugin for MachinePartEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MachinePartRequest>();
    }
}
