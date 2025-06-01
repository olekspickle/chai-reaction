use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

#[derive(Component)]
pub struct SceneCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
}
