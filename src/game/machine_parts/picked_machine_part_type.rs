use crate::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct PickedMachinePartType(pub MachinePartType);

pub struct PickedMachinePartTypePlugin;

impl Plugin for PickedMachinePartTypePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PickedMachinePartType>();
    }
}