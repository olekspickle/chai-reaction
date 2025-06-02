use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use crate::prelude::*;
use crate::single_else_return;

#[derive(Resource, Default)]
pub struct CursorWorldPosition(Option<Vec2>);

pub struct InGameMouseInputPlugin;

impl Plugin for InGameMouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPosition>()
            .add_systems(Update, (
            (update_cursor_in_game_world,
                send_part_spawn_request.run_if(input_just_pressed(MouseButton::Left)))
            .chain()
            .run_if(in_state(Screen::Gameplay)),
        ));
    }
}

fn update_cursor_in_game_world(
    mut cursor: ResMut<CursorWorldPosition>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = single_else_return!(windows);
    let (camera, transform) = single_else_return!(camera);

    if let Some(screen_position) = window.cursor_position() {
        let maybe_world_position = camera
            .viewport_to_world(transform, screen_position)
            .map(|ray_3d| ray_3d.origin.truncate());
        if let Ok(world_position) = maybe_world_position {
            cursor.0 = Some(world_position);
            return;
        }
    }
    cursor.0 = None;
}

fn send_part_spawn_request(
    mut machine_part_request_writer: EventWriter<MachinePartRequest>,
    cursor: Res<CursorWorldPosition>,
){
    if let Some(world_position) = cursor.0{
        machine_part_request_writer.write(MachinePartRequest::SpawnMachinePart(
           MachinePartSpawnRequest{
               location: world_position.extend(MACHINE_PARTS_BASIC_Z_LAYER),
               part_type: MachinePartType::Scale
           }
        ));
    }
}