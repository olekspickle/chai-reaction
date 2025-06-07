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
            listen_to_spawn_requests.run_if(resource_exists::<MachinePartConfigByType>)
                .run_if(resource_exists::<AudioSources>),
        );
    }
}

fn listen_to_spawn_requests(
    mut request_listener: EventReader<MachinePartRequest>,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
    mut available_zen_points: ResMut<AvailableZenPoints>,
    mut commands: Commands,
    editor_mode: Res<EditorMode>,
    sounds: Res<AudioSources>,
    settings: Res<Settings>,
    #[cfg(debug_assertions)] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(debug_assertions)] mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for spawn_request in
        read_single_field_variant!(request_listener, MachinePartRequest::SpawnMachinePart)
    {
        if let Some(part_config) = machine_part_config_by_type
            .0
            .get(&spawn_request.part_type.name)
        {
            if spawn_request.initial_part
                || editor_mode.0
                || available_zen_points
                    .buy_if_affordable(part_config.cost)
                    .done()
            {
                //DEBUG
                info!("Approved spawn request {:?}", spawn_request);

                #[cfg(not(debug_assertions))]
                let spawned = part_config.spawn(spawn_request.part_type.clone(), &mut commands);
                #[cfg(debug_assertions)]
                let spawned = part_config.spawn(
                    spawn_request.part_type.clone(),
                    &mut commands,
                    &sounds,
                    &settings,
                    &mut meshes,
                    &mut materials,
                );

                let source = sounds.place_piece.clone();
                let vol = settings.sound.general * settings.sound.sfx;
                commands.spawn(sfx(source, vol));

                if spawn_request.initial_part {
                    commands.entity(spawned).insert(IsInitialPart);
                }
            }
        }
    }
}
