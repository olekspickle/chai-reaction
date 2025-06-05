use crate::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_event::<OnBack>()
        .add_event::<OnGoTo>()
        .add_event::<OnNewModal>()
        .add_event::<OnPopModal>()
        .add_event::<OnLevelPartSpawn>()
        .add_event::<OnPauseToggle>()
        .add_event::<OnClearModals>()
        .add_event::<OnDebugUiToggle>()
        .add_systems(Update, trigger_input_dispatch);
}

#[derive(Event)]
pub struct OnGoTo(pub Screen);
#[derive(Event)]
pub struct OnBack(pub Screen);
#[derive(Event, Deref)]
pub struct OnNewModal(pub Modal);
#[derive(Event)]
pub struct OnPopModal;
#[derive(Event)]
pub struct OnClearModals;
#[derive(Event)]
pub struct OnPauseToggle;
#[derive(Event)]
pub struct OnDebugUiToggle;
#[derive(Event)]
pub struct OnLevelPartSpawn {
    pub pos: Vec2,
    pub level: GameLevel,
    pub part: LevelPart,
}

fn trigger_input_dispatch(
    mut commands: Commands,
    screen: Res<State<Screen>>,
    settings: Res<Settings>,
    action: Query<&ActionState<Action>>,
) -> Result {
    let state = action.single()?;

    if state.just_pressed(&Action::TogglePause) {
        commands.trigger(OnPauseToggle);
    }
    if state.just_pressed(&Action::ToggleUiDebug) {
        commands.trigger(OnDebugUiToggle);
    }
    if state.just_pressed(&Action::Back) {
        match screen.get() {
            Screen::Splash | Screen::Title | Screen::Loading => {}
            _ => {
                let last = settings.last_screen.clone();
                commands.trigger(OnBack(last));
            }
        }
    }

    Ok(())
}
