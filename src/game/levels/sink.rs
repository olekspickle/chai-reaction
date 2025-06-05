use super::*;
use crate::PrimaryWindow;

pub fn spawn_sink_scene(mut windows: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    for window in windows.iter_mut() {
        let (h, w) = (window.height(), window.width());
        commands.trigger(OnLevelPartSpawn {
            pos: Vec2::new(w / 3.0, h / 3.0),
            level: GameLevel::Sink,
            part: LevelPart::Shelf,
        });
        commands.trigger(OnLevelPartSpawn {
            pos: Vec2::new(-200.0, 100.0),
            level: GameLevel::Sink,
            part: LevelPart::BigTable,
        });
        commands.trigger(OnLevelPartSpawn {
            pos: Vec2::new(0.0, -100.0),
            level: GameLevel::Sink,
            part: LevelPart::TeaCup,
        });
    }
}
