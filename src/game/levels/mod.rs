use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

mod sink;

pub fn plugin(app: &mut App) {
    app.insert_state(GameLevel::Start)
        .add_systems(OnEnter(Screen::Gameplay), prepare_levels)
        .add_systems(OnEnter(GameLevel::Sink), sink::spawn_scene)
        .add_systems(OnEnter(Screen::Title), reset_level)
        .add_observer(hit_glass)
        .add_observer(start_emitting)
        .add_observer(spawn_glass);
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameLevel {
    Start,

    Sink,
    GreatGap,
    TimeWindow,
}

#[derive(Component)]
pub struct Glass;
#[derive(Component)]
pub struct Stove;
#[derive(Component)]
pub struct TeaBox;

fn reset_level(mut game_level: ResMut<NextState<GameLevel>>) {
    game_level.set(GameLevel::Start);
}

fn prepare_levels(
    cfg: Res<Config>,
    mut commands: Commands,
    mut game_level: ResMut<NextState<GameLevel>>,
) {
    commands.insert_resource(Gravity(Vec2::NEG_Y * 9.81 * cfg.physics.gravity));
    game_level.set(GameLevel::Sink);
}

fn hit_glass(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(OnGlassHit);
}

fn spawn_glass(
    trigger: Trigger<OnGlassSpawn>,
    textures: Res<Textures>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const RES: Vec2 = Vec2::new(40.0, 80.0);
    const THICK: f32 = 5.0;
    let OnGlassSpawn { pos, level } = trigger.event();

    let material = materials.add(Color::srgb(0.7, 0.3, 0.2));
    let sprite = Sprite {
        image: textures.glass.clone(),
        custom_size: Some(RES),
        ..default()
    };

    let shape = Rectangle::from_size(RES);
    let anchor = commands
        .spawn((
            StateScoped(*level),
            Glass,
            // sprite,
            // shape.collider(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Friction::new(1.0),
            MassPropertiesBundle::from_shape(&shape, 1.0),
            Transform::from_xyz(pos.x, pos.y - RES.y / 2.0, 0.0),
        ))
        .id();

    let shape = Rectangle::new(RES.x, THICK);
    let mesh = meshes.add(shape);
    let floor = commands
        .spawn((
            StateScoped(*level),
            Mesh2d(mesh),
            MeshMaterial2d(material.clone()),
            // Transform::from_xyz(0.0, -RES.y / 2.0, 0.0),
            RigidBody::Dynamic,
            shape.collider(),
            MassPropertiesBundle::from_shape(&shape, 1.0),
        ))
        .id();
    // sink left wall
    let shape = Rectangle::new(THICK, RES.y);
    let mesh = meshes.add(shape);
    let left = commands
        .spawn((
            StateScoped(*level),
            Mesh2d(mesh),
            MeshMaterial2d(material.clone()),
            // Transform::from_xyz(-RES.x / 2.5, RES.y / 2.0, 0.0),
            RigidBody::Dynamic,
            shape.collider(),
            MassPropertiesBundle::from_shape(&shape, 1.0),
        ))
        .id();
    // glass right wall
    let shape = Rectangle::new(THICK, RES.y);
    let mesh = meshes.add(shape);
    let right = commands
        .spawn((
            StateScoped(*level),
            Mesh2d(mesh),
            MeshMaterial2d(material.clone()),
            // Transform::from_xyz(RES.x / 2.5, RES.y / 2.0, 0.0),
            RigidBody::Dynamic,
            shape.collider(),
            MassPropertiesBundle::from_shape(&shape, 1.0),
        ))
        .id();
    commands
        .spawn(FixedJoint::new(anchor, floor).with_local_anchor_1(Vec2::new(0.0, -RES.y / 2.5)));
    commands.spawn(FixedJoint::new(anchor, left).with_local_anchor_1(Vec2::new(-RES.x / 2.0, 0.0)));
    commands.spawn(FixedJoint::new(anchor, right).with_local_anchor_1(Vec2::new(RES.x / 2.0, 0.0)));
    // shelf
    let shape = Rectangle::new(RES.x * 6.0, 20.0);
    let mesh = meshes.add(shape);
    commands.spawn((
        StateScoped(*level),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(pos.x, pos.y - RES.y * 2.5, 0.0),
        RigidBody::Static,
        shape.collider(),
    ));

    // commands.spawn((
    //     StateScoped(GameLevel::Sink),
    //     ParticleEmitter::new(
    //         ParticleKind::Water,
    //         10.0,  // Spawn 10 particles per second
    //         0.0,   // Min initial speed
    //         0.0,   // Max initial speed
    //         45.0,  // Min angle (degrees, e.g., 60 = upwards right)
    //         160.0, // Max angle (degrees, e.g., 120 = upwards left)
    //         100.0, // live for 100s
    //         1.0,   // Normal gravity effect
    //     ),
    //     Transform::from_xyz(pos.x, pos.y + RES.y / 2.0, 0.0),
    // ));

    // commands
    //         .spawn((
    //             StateScoped(*level),
    //             Glass,
    //             sprite,
    //             // shape.collider(),
    //             RigidBody::Dynamic,
    //             LockedAxes::ROTATION_LOCKED,
    //             Friction::new(1.0),
    //             MassPropertiesBundle::from_shape(&shape, 1.0),
    //             Transform::from_xyz(pos.x, pos.y - RES.y / 2.0, 0.0),
    //         ))
    //         .with_children(|parent| {
    //             // floor
    //             let shape = Rectangle::new(RES.x, THICK);
    //             let mesh = meshes.add(shape);
    //             parent.spawn((
    //                 StateScoped(*level),
    //                 Mesh2d(mesh),
    //                 MeshMaterial2d(material.clone()),
    //                 Transform::from_xyz(0.0, -RES.y / 2.0, 0.0),
    //                 RigidBody::Dynamic,
    //                 shape.collider(),
    //                 MassPropertiesBundle::from_shape(&shape, 1.0),
    //             ));
    //             // glass left wall
    //             let shape = Rectangle::new(THICK, RES.y);
    //             let mesh = meshes.add(shape);
    //             parent.spawn((
    //                 StateScoped(*level),
    //                 Mesh2d(mesh),
    //                 MeshMaterial2d(material.clone()),
    //                 Transform::from_xyz(-RES.x / 2.5, RES.y / 2.0, 0.0),
    //                 RigidBody::Dynamic,
    //                 shape.collider(),
    //                 MassPropertiesBundle::from_shape(&shape, 1.0),
    //             ));
    //             // glass right wall
    //             let shape = Rectangle::new(THICK, RES.y);
    //             let mesh = meshes.add(shape);
    //             parent.spawn((
    //                 StateScoped(*level),
    //                 Mesh2d(mesh),
    //                 MeshMaterial2d(material.clone()),
    //                 Transform::from_xyz(RES.x / 2.5, RES.y / 2.0, 0.0),
    //                 RigidBody::Dynamic,
    //                 shape.collider(),
    //             ));
    //             // shelf
    //             let shape = Rectangle::new(RES.x * 4.0, 20.0);
    //             let mesh = meshes.add(shape);
    //             parent.spawn((
    //                 StateScoped(*level),
    //                 Mesh2d(mesh),
    //                 MeshMaterial2d(material),
    //                 Transform::from_xyz(0.0, -RES.y * 2.0, 0.0),
    //                 RigidBody::Static,
    //                 shape.collider(),
    //             ));
    //             parent.spawn((
    //                 StateScoped(GameLevel::Sink),
    //                 ParticleEmitter::new(
    //                     ParticleKind::Water,
    //                     10.0,  // Spawn 10 particles per second
    //                     0.0,   // Min initial speed
    //                     0.0,   // Max initial speed
    //                     45.0,  // Min angle (degrees, e.g., 60 = upwards right)
    //                     160.0, // Max angle (degrees, e.g., 120 = upwards left)
    //                     100.0, // live for 100s
    //                     1.0,   // Normal gravity effect
    //                 ),
    //                 // Transform::from_xyz(pos.x, pos.y + RES.y / 2.0, 0.0),
    //             ));
    //         });
    //
}

fn start_emitting(
    _: Trigger<OnGlassHit>,
    mut commands: Commands,
    glass: Query<Entity, With<Glass>>,
) {
    for e in glass.iter() {
        commands.entity(e).insert((
            StateScoped(GameLevel::Sink),
            ParticleEmitter::new(
                ParticleKind::Water,
                10.0,  // Spawn 10 particles per second
                0.0,   // Min initial speed
                0.0,   // Max initial speed
                45.0,  // Min angle (degrees, e.g., 60 = upwards right)
                160.0, // Max angle (degrees, e.g., 120 = upwards left)
                100.0, // live for 100s
                1.0,   // Normal gravity effect
            ),
        ));
    }
}

fn spawn_stove(
    trigger: Trigger<OnStoveSpawn>,
    textures: Res<Textures>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const RES: Vec2 = Vec2::new(100.0, 100.0);
    let OnStoveSpawn { pos, level } = trigger.event();

    commands.spawn((
        StateScoped(*level),
        Glass,
        Sprite {
            image: textures.glass.clone(),
            custom_size: Some(RES),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));

    let shape = Rectangle::from_size(RES);
    let mesh = meshes.add(shape);
    let material = materials.add(Color::WHITE);
    commands.spawn((
        StateScoped(*level),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(pos.x, pos.y - RES.y, 0.0),
        RigidBody::Static,
        shape.collider(),
    ));
}
