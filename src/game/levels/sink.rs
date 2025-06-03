use super::*;

pub fn spawn_scene(
    cfg: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pos = Vec2::new(200.0, 200.0);
    commands.trigger(OnGlassSpawn {
        pos,
        level: GameLevel::Sink,
    });
    // Water vessel
}
