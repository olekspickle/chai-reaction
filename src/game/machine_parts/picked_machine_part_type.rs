use crate::prelude::*;

#[derive(Resource, Debug, Default)]
pub enum PickingState {
    #[default]
    None,
    Placing(MachinePartType),
    Erasing,
}



pub struct PickedMachinePartTypePlugin;

impl Plugin for PickedMachinePartTypePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PickingState>();
    }
}
