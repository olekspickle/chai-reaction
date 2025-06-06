use crate::prelude::*;
use crate::read_single_field_variant;
use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

pub struct MachinePartSpawnerPlugin;

#[derive(Component)]
pub struct IsInitialPart;

impl Plugin for MachinePartSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_to_spawn_requests.run_if(resource_exists::<MachinePartConfigByType>),
        );
    }
}

fn listen_to_spawn_requests(
    mut request_listener: EventReader<MachinePartRequest>,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
    mut available_zen_points: ResMut<AvailableZenPoints>,
    mut commands: Commands,
) {
    for spawn_request in
        read_single_field_variant!(request_listener, MachinePartRequest::SpawnMachinePart)
    {
        if let Some(part_config) = machine_part_config_by_type
            .0
            .get(&spawn_request.part_type.name)
        {
            if spawn_request.initial_part || available_zen_points
                .buy_if_affordable(part_config.cost)
                .done()
            {
                //DEBUG
                info!("Approved spawn request {:?}", spawn_request);

                let spawned = part_config.spawn(
                    spawn_request.part_type.clone(),
                    &mut commands,
                );
                if spawn_request.initial_part {
                    commands.entity(spawned).insert(IsInitialPart);
                }
            }
        }
    }
}
