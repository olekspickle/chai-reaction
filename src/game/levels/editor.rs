use crate::prelude::*;
use avian2d::prelude::Gravity;
use bevy::prelude::*;

pub struct LevelEditor(pub String);

#[derive(Resource)]
struct LevelPath(String);

#[derive(Resource)]
pub struct EditorLevel(pub Handle<LevelConfig>);

impl Plugin for LevelEditor {
    fn build(&self, app: &mut App) {
        let path = format!("assets/levels/{}.ron", self.0.clone());
        app.insert_resource(LevelPath(path))
            .add_systems(Update, save.run_if(resource_exists::<EditorLevel>))
            .add_systems(OnEnter(Screen::Gameplay), load);
    }
}

fn load(
    mut commands: Commands,
    level_path: Res<LevelPath>,
    mut level_configs: ResMut<Assets<LevelConfig>>,
    cfg: Res<Config>,
) {
    let handle = if let Some(config) = std::fs::read_to_string(&level_path.0)
        .ok()
        .and_then(|d| ron::from_str::<LevelConfig>(&d).ok())
    {
        level_configs.add(config)
    } else {
        level_configs.add(LevelConfig::default())
    };
    commands.insert_resource(LoadedLevel(handle.clone()));
    commands.insert_resource(EditorLevel(handle));
    commands.insert_resource(Gravity(Vec2::NEG_Y * 9.81 * cfg.physics.gravity));
}

fn save(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    level_path: Res<LevelPath>,
    machine_parts: Query<&MachinePartType, With<SpawnedMachinePart>>,
    editor_level: Res<EditorLevel>,
    level_configs: Res<Assets<LevelConfig>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        let mut level = level_configs.get(&editor_level.0).unwrap().clone();
        level.initial_machine_parts.clear();
        for part_type in &machine_parts {
            level.initial_machine_parts.push(part_type.clone());
        }
        std::fs::write(
            &level_path.0,
            ron::ser::to_string_pretty(&level, Default::default()).unwrap(),
        )
        .unwrap();
    }
}
