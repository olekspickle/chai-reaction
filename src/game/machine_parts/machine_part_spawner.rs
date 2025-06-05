use crate::prelude::*;
use crate::read_single_field_variant;
use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

pub struct MachinePartSpawnerPlugin;

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
            if available_zen_points
                .buy_if_affordable(part_config.cost)
                .done()
            {
                //DEBUG
                info!("Approved spawn request {:?}", spawn_request);

                part_config.spawn(
                    spawn_request.part_type.clone(),
                    &mut commands,
                );
                /*
                commands.spawn((
                    Transform::from_translation(spawn_request.location),
                    spawn_request.part_type.clone(),
                    Sprite{
                        image: part_config.sprite.clone(),
                        ..default()
                    },
                    if part_config.is_dynamic {
                        RigidBody::Dynamic
                    } else {
                        RigidBody::Static
                    },
                    Pickable::default(),
                )).observe(handle_erase_click).with_children(|parent| {
                    for collider in &part_config.colliders {
                        parent.spawn(
                            Collider::from(SharedShape::new(collider.clone())),
                        );
                    }
                    if let Some(sprite) = &part_config.background_sprite {
                        parent.spawn((
                            Transform::from_xyz(0.0, 0.0, -100.0),
                            Sprite{
                                image: sprite.clone(),
                                ..default()
                            },
                        ));
                    }
                    if let Some(flow_field) = &part_config.flow_field_data {
                        // Spawn the FlowField component
                        let texture = flow_field.texture.clone();
                        parent.spawn((
                                FlowField { texture: texture.clone() },
                                flow_field.collider.clone(),
                                Sprite{
                                    image: texture.clone(),
                                    color: Color::linear_rgba(1.0,1.0,1.0,0.5),
                                    ..default()
                                },
                        ));
                    }
                });
                */
            }
        }
    }
}
