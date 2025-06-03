use crate::prelude::*;
use avian2d::prelude::*;
use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
};

mod sink;

pub fn plugin(app: &mut App) {
    app.insert_state(GameLevel::Start)
        .add_systems(OnEnter(Screen::Gameplay), prepare_levels)
        .add_systems(OnEnter(GameLevel::Sink), sink::spawn_scene)
        .add_observer(hit_glass)
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
            shape.collider(),
            RigidBody::Kinematic,
            MassPropertiesBundle::from_shape(&shape, 1.0),
            Transform::from_xyz(pos.x, pos.y - RES.y / 2.0, 0.0),
        ))
        .id();

    let shape = Rectangle::new(RES.x - 5.0, 2.0);
    let mesh = meshes.add(shape);
    let floor = commands
        .spawn((
            StateScoped(*level),
            Glass,
            Mesh2d(mesh),
            MeshMaterial2d(material.clone()),
            // Transform::from_xyz(0.0, -RES.y / 2.0, 0.0),
            RigidBody::Dynamic,
            shape.collider(),
            MassPropertiesBundle::from_shape(&shape, 1.0),
        ))
        .id();
    // sink left wall
    let shape = Rectangle::new(2.0, RES.y);
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
    let shape = Rectangle::new(2.0, RES.y);
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
    commands.spawn(FixedJoint::new(anchor, left).with_local_anchor_1(Vec2::new(-RES.x / 2.5, 0.0)));
    commands.spawn(FixedJoint::new(anchor, right).with_local_anchor_1(Vec2::new(RES.x / 2.5, 0.0)));
    // shelf
    let shape = Rectangle::new(RES.x * 3.0, 20.0);
    let mesh = meshes.add(shape);
    commands.spawn((
        StateScoped(*level),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(pos.x, pos.y - RES.y * 1.2, 0.0),
        RigidBody::Static,
        shape.collider(),
    ));
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
