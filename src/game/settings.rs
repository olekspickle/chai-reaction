use crate::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Settings>();
    app.add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, spawn_player_input_map)
        .add_systems(
            OnEnter(Screen::Title),
            inject_settings_from_cfg.run_if(resource_exists::<Config>),
        );
}

#[derive(Resource)]
pub struct Settings {
    pub sound: Sound,

    // game state things
    /// Modal stack. kudo for the idea to @skyemakesgames
    /// Only relevant in [`Screen::Gameplay`]
    pub modals: Vec<Modal>,
    pub paused: bool,
    pub last_screen: Screen,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            last_screen: Screen::Title,
            sound: Sound::default(),
            modals: vec![],
            paused: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Modal {
    Main,
    Settings,
}

fn inject_settings_from_cfg(mut commands: Commands, cfg: Res<Config>) {
    commands.insert_resource(Settings {
        sound: cfg.sound.clone(),
        ..Default::default()
    });
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    One,
    Two,

    TogglePause,
    ToggleUiDebug,
    Back,

    RotateCcw,
    RotateCw,
    Flip,
}

fn spawn_player_input_map(mut commands: Commands) {
    let mut input_map = InputMap::default();

    input_map.insert(Action::One, KeyCode::KeyA);
    input_map.insert(Action::Two, KeyCode::KeyD);

    input_map.insert(Action::ToggleUiDebug, KeyCode::Backquote);
    input_map.insert(Action::TogglePause, KeyCode::KeyP);
    input_map.insert(Action::Back, KeyCode::Escape);

    input_map.insert(Action::RotateCcw, KeyCode::KeyQ);
    input_map.insert(Action::RotateCw, KeyCode::KeyE);
    input_map.insert(Action::Flip, KeyCode::KeyF);

    commands.spawn(input_map);
}
