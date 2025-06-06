use bevy::prelude::*;
use crate::prelude::*;

pub struct LevelEditor(pub String);

#[derive(Resource)]
struct LevelPath(String);

impl Plugin for LevelEditor {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelPath(self.0.clone()))
            .add_systems(Update, save);
    }
}

fn save(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    level_path: Res<LevelPath>,
    machine_parts: Query<(&Transform, &MachinePartType), With<SpawnedMachinePart>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        let mut level = LevelConfig::default();
        for (transform, part_type) in &machine_parts {
            level.initial_machine_parts.push(PartPlacement {
                part_type: part_type.0.clone(),
                position: transform.translation.truncate(),
                ..default()
            });
        }
        std::fs::write(&level_path.0, &ron::ser::to_string_pretty(&level, Default::default()).unwrap()).unwrap();
    }
}
