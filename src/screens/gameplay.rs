//! The screen state for the main gameplay.

use super::*;
use crate::{game::input_dispatch::*, game::physics::PhysicsState, screens::settings};
use bevy::ui::Val::*;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(crate::game::plugin)
        .add_systems(OnEnter(Screen::Gameplay), spawn_gameplay_ui)
        .add_systems(
            Update,
            (change_score.run_if(in_state(Screen::Gameplay)), restart),
        )
        .init_resource::<ModifiedLevel>()
        .add_observer(trigger_menu_toggle_on_esc)
        .add_observer(add_new_modal)
        .add_observer(pop_modal)
        .add_observer(clear_modals);
}

#[derive(Component)]
pub struct GameplayUi;
#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct MenuModal;
#[derive(Component)]
pub struct SettingsModal;
#[derive(Component)]
pub struct GameoverModal;

fn spawn_gameplay_ui(mut cmds: Commands, textures: Res<Textures>) {
    let (play, exit, glass) = (
        textures.play.clone(),
        textures.exit.clone(),
        textures.cup.clone(),
    );
    let score = Opts::new("score: 0")
        .border_radius(Px(0.0))
        .color(Color::BLACK);
    let nav_opts = Opts::default()
        .image(exit)
        .width(Vw(5.0))
        .height(Vw(5.0))
        .bg_color(TRANSPARENT)
        .ui_palette(UiPalette::all(TRANSPARENT).hovered((TRANSPARENT, WHITEISH)));
    cmds.spawn((
        StateScoped(Screen::Gameplay),
        GameplayUi,
        ui_root("gameplay ui"),
        children![
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    left: Vw(40.0),
                    top: Px(0.0),
                    height: Vw(5.0),
                    width: Vw(20.0),
                    ..Default::default()
                },
                BackgroundColor(TRANSLUCENT),
                children![
                    btn(nav_opts.clone(), to::title),
                    btn(nav_opts.clone().image(play), toggle_physics),
                    btn(nav_opts.image(glass), init_level),
                ]
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Px(0.0),
                    right: Px(0.0),
                    padding: UiRect::axes(Vw(2.0), Vw(1.0)),
                    ..default()
                },
                TimeLabel,
                BackgroundColor(YELLOW),
                children![label(score)]
            )
        ],
    ));
}

#[derive(Component)]
pub struct ScoreTimer(pub Timer);
#[derive(Component)]
pub struct TimeLabel;

// TODO: Gameplay UI and systems

fn change_score(// mut commands: Commands,
    // mut score: ResMut<Score>,
    // mut score_label: Query<&mut Text, With<TimeLabel>>,
) -> Result {
    /*
    for counter in counter.iter() {
        let mut label = score_label.single_mut()?;
        score.0 = (counter.0 * 10) as i32;
        label.0 = format!("score: 0", score.0);
    }
    */

    Ok(())
}

fn restart(
    level: ResMut<State<GameLevel>>,
    mut next: ResMut<NextState<GameLevel>>,
    action: Query<&ActionState<Action>>,
) {
    if let Ok(state) = action.single() {
        if state.just_pressed(&Action::Restart) {
            let current = level.get();
            info!("pressed R: {current:?}");
            next.set(GameLevel::Start);
            next.set(current.clone());
        }
    }
}

#[derive(Resource, Default)]
pub struct ModifiedLevel(pub Option<Vec<MachinePartType>>);

fn toggle_physics(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    physics_state: ResMut<State<PhysicsState>>,
    mut next: ResMut<NextState<PhysicsState>>,
    machine_parts: Query<&MachinePartType, With<SpawnedMachinePart>>,
    mut machine_part_request_writer: EventWriter<MachinePartRequest>,
    mut modified_level: ResMut<ModifiedLevel>,
) {
    match physics_state.get() {
        PhysicsState::Paused => {
            let mut parts = vec![];
            for part_type in &machine_parts {
                parts.push(part_type.clone());
            }
            modified_level.0 = Some(parts);
            next.set(PhysicsState::Running)
        }
        PhysicsState::Running => {
            if let Some(parts) = &modified_level.0 {
                commands.queue(ClearLevel);
                for part in parts {
                    machine_part_request_writer.write(MachinePartRequest::SpawnMachinePart(
                        MachinePartSpawnRequest {
                            location: part.context.position,
                            part_type: part.clone(),
                            initial_part: true,
                        },
                    ));
                }
            }
            next.set(PhysicsState::Paused)
        }
    }
}

fn init_level(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut physics_state: ResMut<NextState<PhysicsState>>,
) {
    commands.queue(InitLevel);
    physics_state.set(PhysicsState::Paused);
}

// Modals and navigation

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
