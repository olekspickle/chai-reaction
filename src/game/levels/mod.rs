use std::env;
use crate::prelude::*;
use avian2d::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::loading::LoadResource;

use serde::{Serialize, Deserialize};

mod sink;
#[cfg(feature="dev")]
mod editor;

#[derive(Resource)]
pub struct EditorMode(pub bool);


#[derive(Default, Asset, Resource, Reflect, Clone, Debug)]
pub struct LevelList(pub Vec<Handle<LevelConfig>>);

#[derive(Resource)]
pub struct LoadedLevel(pub Handle<LevelConfig>);

pub fn plugin(app: &mut App) {
    app.insert_state(GameLevel::Start)
        .enable_state_scoped_entities::<GameLevel>()
        .add_systems(OnEnter(GameLevel::Sink), sink::spawn_sink_scene)
        .add_systems(OnEnter(Screen::Title), reset_level)
        .add_systems(Update, init_level.run_if(resource_exists_and_changed::<LoadedLevel>))
        .add_observer(spawn_level_part)
        .add_plugins(RonAssetPlugin::<LevelConfig>::new(&["level.ron"]))
        .register_asset_loader(LevelListLoader)
        .load_resource_from_path::<LevelList>("levels.ron");

    let mut in_editor = false;
    #[cfg(feature="dev")]
    if let Some(path) = env::args().nth(1) {
        app.add_plugins(editor::LevelEditor(path.to_string()));
        in_editor = true;
    }

    if in_editor {
        app.insert_resource(EditorMode(true));
    } else {
        app.add_systems(OnEnter(Screen::Gameplay), prepare_levels);
        app.insert_resource(EditorMode(false));
    }
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

pub fn prepare_levels(
    cfg: Res<Config>,
    mut commands: Commands,
    level_list: Res<LevelList>,
) {
    commands.insert_resource(Gravity(Vec2::NEG_Y * 9.81 * cfg.physics.gravity));
    commands.insert_resource(LoadedLevel(level_list.0[0].clone()));
}

fn init_level(
    mut commands: Commands,
    loaded_level: Res<LoadedLevel>,
    level_configs: Res<Assets<LevelConfig>>,
    mut machine_part_request_writer: EventWriter<MachinePartRequest>,
    existing_parts: Query<Entity, With<SpawnedMachinePart>>,
) {
    for entity in &existing_parts {
        commands.entity(entity).despawn();
    }

    if let Some(config) = level_configs.get(&loaded_level.0) {
        for part in &config.initial_machine_parts {
            machine_part_request_writer.write(MachinePartRequest::SpawnMachinePart(
                MachinePartSpawnRequest {
                    location: part.context.position,
                    part_type: part.clone(),
                    force: true,
                },
            ));
        }
    }
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


#[derive(Default, Asset, Reflect, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    pub name: String,
    pub zen_points: u32,
    pub available_machine_parts: Vec<String>,
    pub initial_machine_parts: Vec<MachinePartType>,
}



#[derive(Default)]
struct LevelListLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum LevelListLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

impl AssetLoader for LevelListLoader {
    type Asset = LevelList;
    type Settings = ();
    type Error = LevelListLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let levels = ron::de::from_bytes::<Vec<String>>(&bytes)?;

        let mut level_list = LevelList::default();
        for path in levels {
            level_list.0.push(load_context
                .loader()
                .with_static_type()
                .load::<LevelConfig>(path.clone()));
        }

        Ok(level_list)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
