use crate::prelude::*;
use bevy::prelude::*;
use crate::read_single_field_variant;

pub struct MachinePartSpawnerPlugin;

impl Plugin for MachinePartSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_to_spawn_requests);
    }
}

fn listen_to_spawn_requests(
    mut request_listener: EventReader<MachinePartRequest>,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
    mut available_zen_points: ResMut<AvailableZenPoints>,
    mut commands: Commands
){
    for spawn_request in read_single_field_variant!(request_listener, MachinePartRequest::SpawnMachinePart){
        if let Some(part_config) = machine_part_config_by_type.0.get(&spawn_request.part_type){
            if available_zen_points.buy_if_affordable(part_config.cost).done(){
                //DEBUG
                info!("Approved spawn request {:?}", spawn_request);

                commands.spawn((
                    Transform::from_translation(spawn_request.location),
                    spawn_request.part_type,
                    Sprite{
                        image: part_config.sprite.clone(),
                        ..default()
                    }
                ));
            }
        }
    }
}