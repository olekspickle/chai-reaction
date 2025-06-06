use bevy::prelude::*;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_camera.run_if(resource_added::<Config>));
}

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
) {
    let screen_width = 16.0 * config.screen_size_in_tiles.x as f32;
    let screen_height = 16.0 * config.screen_size_in_tiles.y as f32 + 64.0;
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: screen_width,
                min_height: screen_height,
            },
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
        IsDefaultUiCamera
    ));

    // Mask overlay
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(-5000.0 - screen_width*0.5, 0.0, 100.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(5000.0 + screen_width*0.5, 0.0, 100.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.0, 5000.0 + screen_height*0.5, 100.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.0, -5000.0 - screen_height*0.5, 100.0),
    ));
}
