//! The screen state for the main gameplay.

use super::*;
use crate::{
    game::{input_dispatch::*, tea::TeaCounter},
    screens::settings,
};
use bevy::ui::Val::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(crate::game::plugin)
        .add_systems(OnEnter(Screen::Gameplay), spawn_gameplay_ui)
        // TODO: Update systems
        .add_systems(Update, change_score.run_if(in_state(Screen::Gameplay)))
        .add_observer(trigger_menu_toggle_on_esc)
        .add_observer(add_new_modal)
        .add_observer(pop_modal)
        .add_observer(clear_modals);
}

#[derive(Component)]
pub struct DevUi;
#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct MenuModal;
#[derive(Component)]
pub struct SettingsModal;
#[derive(Component)]
pub struct GameoverModal;

fn spawn_gameplay_ui(mut cmds: Commands, settings: Res<Settings>) {
    cmds.spawn((
        StateScoped(Screen::Gameplay),
        DevUi,
        Node {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        children![(
            Node {
                position_type: PositionType::Absolute,
                top: Px(0.0),
                right: Px(0.0),
                ..default()
            },
            ScoreLabel,
            label("0")
        )],
    ));
}

#[derive(Component)]
pub struct ScoreLabel;

// TODO: Gameplay UI and systems

fn change_score(
    mut commands: Commands,
    mut score: ResMut<Score>,
    counter: Query<&TeaCounter, Changed<TeaCounter>>,
    mut score_label: Query<&mut Text, With<ScoreLabel>>,
) -> Result {
    for counter in counter.iter() {
        let mut label = score_label.single_mut()?;
        score.0 = (counter.0 * 10) as i32;
        label.0 = format!("{}", score.0);
    }

    Ok(())
}

fn click_to_menu(_: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(OnGoTo(Screen::Title));
}
fn click_pop_modal(_: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(OnPopModal);
}
fn click_spawn_settings(_: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(OnNewModal(Modal::Settings));
}

fn trigger_menu_toggle_on_esc(
    _: Trigger<OnBack>,
    mut cmds: Commands,
    screen: Res<State<Screen>>,
    settings: ResMut<Settings>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }
    if settings.modals.is_empty() {
        cmds.trigger(OnNewModal(Modal::Main));
    } else {
        cmds.trigger(OnPopModal);
    }
}

fn add_new_modal(
    trig: Trigger<OnNewModal>,
    score: Res<Score>,
    screen: Res<State<Screen>>,
    mut cmds: Commands,
    mut settings: ResMut<Settings>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }

    if settings.modals.is_empty() {
        cmds.trigger(OnPauseToggle);
    }

    // despawn all previous modals
    cmds.trigger(OnClearModals);
    let OnNewModal(modal) = trig.event();
    match modal {
        Modal::Main => cmds.spawn(menu_modal()),
        Modal::Settings => cmds.spawn(settings_modal()),
        Modal::Gameover => cmds.spawn(gameover_modal(score.0)),
    };

    settings.modals.push(modal.clone());
}

fn pop_modal(
    _: Trigger<OnPopModal>,
    mut cmds: Commands,
    screen: Res<State<Screen>>,
    mut settings: ResMut<Settings>,
    menu_marker: Query<Entity, With<MenuModal>>,
    settings_marker: Query<Entity, With<SettingsModal>>,
    gameover_marker: Query<Entity, With<GameoverModal>>,
) {
    if Screen::Gameplay != *screen.get() {
        return;
    }

    // just a precaution
    assert!(!settings.modals.is_empty());

    let popped = settings.modals.pop().expect("popped modal with empty list");
    match popped {
        Modal::Main => {
            if let Ok(menu) = menu_marker.single() {
                cmds.entity(menu).despawn();
            }
        }
        Modal::Settings => {
            if let Ok(settings) = settings_marker.single() {
                cmds.entity(settings).despawn();
            }
        }
        Modal::Gameover => {
            if let Ok(gameover) = gameover_marker.single() {
                cmds.entity(gameover).despawn();
            }
        }
    }

    // respawn next in the modal stack
    if let Some(modal) = settings.modals.last() {
        match modal {
            Modal::Main => {
                cmds.spawn(menu_modal());
            }
            Modal::Settings => {
                cmds.spawn(settings_modal());
            }
            _ => {}
        }
    }

    if settings.modals.is_empty() {
        cmds.trigger(OnPauseToggle);
    }
}

fn clear_modals(
    _: Trigger<OnClearModals>,
    mut cmds: Commands,
    settings: ResMut<Settings>,
    menu_marker: Query<Entity, With<MenuModal>>,
    settings_marker: Query<Entity, With<SettingsModal>>,
    gameover_marker: Query<Entity, With<GameoverModal>>,
) {
    for m in &settings.modals {
        match m {
            Modal::Main => {
                if let Ok(menu) = menu_marker.single() {
                    cmds.entity(menu).despawn();
                }
            }
            Modal::Settings => {
                if let Ok(settings) = settings_marker.single() {
                    cmds.entity(settings).despawn();
                }
            }
            Modal::Gameover => {
                if let Ok(gameover) = gameover_marker.single() {
                    cmds.entity(gameover).despawn();
                }
            }
        }
    }
}

fn settings_modal() -> impl Bundle {
    (
        StateScoped(Screen::Gameplay),
        SettingsModal,
        BackgroundColor(TRANSLUCENT),
        settings::ui(),
    )
}

fn menu_modal() -> impl Bundle {
    let opts = Opts::new("Settings")
        .width(Vw(15.0))
        .padding(UiRect::axes(Vw(2.0), Vw(0.5)));
    (
        StateScoped(Screen::Gameplay),
        MenuModal,
        ui_root("In game menu"),
        children![(
            BorderColor(WHITEISH),
            BackgroundColor(TRANSLUCENT),
            Node {
                border: UiRect::all(Px(2.0)),
                padding: UiRect::all(Vw(10.0)),
                ..default()
            },
            children![
                (
                    Node {
                        position_type: PositionType::Absolute,
                        top: Px(0.0),
                        right: Px(0.0),
                        ..Default::default()
                    },
                    children![btn_small(Opts::new("x").width(Vw(5.0)), click_pop_modal)]
                ),
                (
                    Node {
                        row_gap: Percent(20.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    children![
                        btn(opts.clone(), click_spawn_settings),
                        btn(opts.text("Main Menu"), click_to_menu)
                    ]
                )
            ]
        )],
    )
}

fn gameover_modal(score: i32) -> impl Bundle {
    (
        StateScoped(Screen::Gameplay),
        ui_root("game over modal"),
        BackgroundColor(TRANSLUCENT),
        #[cfg(target_family = "wasm")]
        children![label(format!("Score: {score}")), btn("Next", next_level)],
        #[cfg(not(target_family = "wasm"))]
        children![
            label(format!("Score: {score}")),
            btn_big("Main Menu", enter_gameplay_screen),
            btn_big("Next Level", next_level),
            btn_big("Exit", exit_app)
        ],
    )
}

fn next_level(_trigger: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<GameLevel>>) {
    next_screen.set(GameLevel::Second);
}

fn enter_gameplay_screen(
    _trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Gameplay);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
