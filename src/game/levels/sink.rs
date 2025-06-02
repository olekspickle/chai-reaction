use super::*;

pub fn spawn_scene(
    cfg: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Gravity(Vec2::NEG_Y * 9.81 * cfg.physics.gravity));
    commands.spawn((
        StateScoped(GameLevel::Sink),
        ParticleEmitter::new(
            ParticleKind::Water,
            10.0,  // Spawn 10 particles per second
            20.0,  // Min initial speed
            50.0,  // Max initial speed
            45.0,  // Min angle (degrees, e.g., 60 = upwards right)
            160.0, // Max angle (degrees, e.g., 120 = upwards left)
            10.0,  // live for 100s
            1.0,   // Normal gravity effect
        ),
        Transform::from_xyz(-150.0, -150.0, 0.0),
    ));
    // Water vessel
    // sink right wall
    let shape = Rectangle::new(10.0, 50.0);
    let mesh = meshes.add(shape);
    let material = materials.add(GRAY);
    commands.spawn((
        StateScoped(GameLevel::Sink),
        WaterBucket,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(200.0, 200.0, 0.0),
        RigidBody::Static,
        shape.collider(),
    ));
    // sink left wall
    let shape = Rectangle::new(10.0, 50.0);
    let mesh = meshes.add(shape);
    let material = materials.add(GRAY);
    commands.spawn((
        StateScoped(GameLevel::Sink),
        WaterBucket,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(-250.0, -200.0, 0.0),
        RigidBody::Static,
        shape.collider(),
    ));
    // sink floor
    let shape = Rectangle::new(200.0, 10.0);
    let mesh = meshes.add(shape);
    let material = materials.add(GRAY);
    commands.spawn((
        StateScoped(GameLevel::Sink),
        WaterBucket,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(-150.0, -220.0, 0.0),
        RigidBody::Static,
        shape.collider(),
    ));
}
