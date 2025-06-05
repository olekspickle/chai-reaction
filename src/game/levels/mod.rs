use crate::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

mod sink;

pub fn plugin(app: &mut App) {
    app.insert_state(GameLevel::Start)
        .enable_state_scoped_entities::<GameLevel>()
        .add_systems(OnEnter(Screen::Gameplay), prepare_levels)
        .add_systems(OnEnter(GameLevel::Sink), sink::spawn_sink_scene)
        .add_systems(OnEnter(Screen::Title), reset_level)
        .add_observer(spawn_level_part);
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameLevel {
    Start,

    Sink,
    GreatGap,
    TimeForWindow,
}

#[derive(Component)]
pub enum LevelPart {
    Shelf,
    BigTable,
    TeaCup,
}
#[derive(Component)]
pub struct Stove;
#[derive(Component)]
pub struct TeaBox;

fn reset_level(mut game_level: ResMut<NextState<GameLevel>>) {
    info!("resetting GameLevel to Start");
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

fn spawn_level_part(
    trigger: Trigger<OnLevelPartSpawn>,
    textures: Res<Textures>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let OnLevelPartSpawn { pos, level, part } = trigger.event();

    match part {
        LevelPart::Shelf => {
            // Shelf
            let shape = Rectangle::new(120.0, 20.0);
            let mesh = meshes.add(shape);
            let material = materials.add(Color::srgb(0.7, 0.3, 0.2));
            commands.spawn((
                StateScoped(*level),
                Mesh2d(mesh),
                MeshMaterial2d(material),
                Transform::from_xyz(pos.x, pos.y, 0.0),
                RigidBody::Static,
                shape.collider(),
            ));
        }
        LevelPart::BigTable => {
            let shape = Rectangle::new(400.0, 150.0);
            let mesh = meshes.add(shape);
            let material = materials.add(Color::srgb(0.7, 0.3, 0.2));
            commands.spawn((
                StateScoped(*level),
                Mesh2d(mesh),
                MeshMaterial2d(material),
                Transform::from_xyz(pos.x, pos.y, 0.0),
                RigidBody::Static,
                shape.collider(),
            ));
        }
        LevelPart::TeaCup => {
            const RES: Vec2 = Vec2::new(40.0, 50.0);
            let sprite = Sprite {
                image: textures.cup.clone(),
                custom_size: Some(RES),
                ..default()
            };
            let shape = Rectangle::from_size(RES);
            commands.spawn((
                StateScoped(*level),
                LevelPart::TeaCup,
                sprite,
                shape.collider(),
                RigidBody::Static,
                Transform::from_xyz(pos.x, pos.y, 0.0),
                Sensor,
            ));
            // Cup table
            let shape = Rectangle::new(100.0, 200.0);
            let mesh = meshes.add(shape);
            let material = materials.add(Color::srgb(0.7, 0.3, 0.2));
            commands.spawn((
                StateScoped(*level),
                Mesh2d(mesh),
                MeshMaterial2d(material),
                Transform::from_xyz(pos.x, pos.y - RES.y * 1.1, 0.0),
                RigidBody::Static,
                shape.collider(),
            ));
        }
    }
}
