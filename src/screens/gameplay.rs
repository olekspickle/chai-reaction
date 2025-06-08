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
            (
                change_score.run_if(in_state(Screen::Gameplay)),
                instant_victory
                    .run_if(resource_exists::<LoadedLevel>.and(resource_exists::<LevelList>)),
            ),
        )
        .init_resource::<ModifiedLevel>()
        .add_observer(trigger_menu_toggle_on_esc)
        .add_observer(add_new_modal)
        .add_observer(pop_modal)
        .add_observer(clear_modals);
}

#[derive(Resource)]
pub struct NextLevel(pub usize);

#[derive(Component)]
pub struct ScoreTimer(pub Timer);
#[derive(Component)]
pub struct GameplayUi;
#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct ScoreLabel;
#[derive(Component)]
pub struct MenuModal;
#[derive(Component)]
pub struct SettingsModal;
#[derive(Component)]
pub struct LevelFinishedModal;

fn spawn_gameplay_ui(mut commands: Commands, textures: Res<Textures>) {
    let (play, exit, reset) = (
        textures.play.clone(),
        textures.exit.clone(),
        textures.reset.clone(),
    );
    let score = Opts::new("TODO: cup goals")
        .border_radius(Px(0.0))
        .color(Color::BLACK);
    let nav_opts = Opts::default()
        .image(exit)
        .width(Vw(5.0))
        .height(Vw(5.0))
        .bg_color(TRANSPARENT)
        .ui_palette(UiPalette::all(TRANSPARENT).hovered((TRANSPARENT, WHITEISH)));
    commands.spawn((
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
                    btn(nav_opts.image(reset), init_level),
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
                ScoreLabel,
                BackgroundColor(YELLOW),
                children![label(score)]
            )
        ],
    ));
}

// TODO: Gameplay UI and systems

fn change_score(// all_sensors: Query<&TeaSensor>,
    // satisfied: Query<Has<Satisfied>, With<TeaSensor>>,
    // mut score_label: Query<&mut Text, With<ScoreLabel>>,
) {
    // let (all, satisfied) = (all_sensors.iter().len(), satisfied.iter().len());
    // if let Ok(mut label) = score_label.single_mut() {
    // label.0 = format!("teas {satisfied}/{all}");
    // }
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
pub struct ModifiedLevel(pub Option<Vec<(MachinePartType, bool)>>);

fn toggle_physics(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    physics_state: ResMut<State<PhysicsState>>,
    mut next: ResMut<NextState<PhysicsState>>,
    machine_parts: Query<(&MachinePartType, Has<IsInitialPart>), With<SpawnedMachinePart>>,
    mut machine_part_request_writer: EventWriter<MachinePartRequest>,
    mut modified_level: ResMut<ModifiedLevel>,
) {
    match physics_state.get() {
        PhysicsState::Paused => {
            let mut parts = vec![];
            for (part_type, is_initial) in &machine_parts {
                parts.push((part_type.clone(), is_initial));
            }
            modified_level.0 = Some(parts);
            next.set(PhysicsState::Running)
        }
        PhysicsState::Running => {
            if let Some(parts) = &modified_level.0 {
                commands.queue(ClearLevel);
                for (part, is_initial) in parts {
                    machine_part_request_writer.write(MachinePartRequest::SpawnMachinePart(
                        MachinePartSpawnRequest {
                            location: part.context.position,
                            part_type: part.clone(),
                            initial_part: *is_initial,
                            free: true,
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

// DEBUG: to test going to next level. recolor_particles panics so to test - comment it out
fn instant_victory(
    mut commands: Commands,
    action: Query<&ActionState<Action>>,
    loaded_level: ResMut<LoadedLevel>,
    level_list: Res<LevelList>,
) {
    if let Ok(state) = action.single() {
        if state.just_pressed(&Action::DebugNextLevel) {
            if let Some(idx) = level_list.0.iter().position(|l| l == &loaded_level.0) {
                let new_idx = idx + 1;
                if new_idx < level_list.0.len() {
                    // save next level id and spawn a modal in gameplay screen
                    commands.insert_resource(NextLevel(new_idx));
                    commands.trigger(OnNewModal(Modal::LevelFinished));
                }
            }
        }
    }
}

fn click_to_next_level(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut loaded_level: ResMut<LoadedLevel>,
    level_list: Res<LevelList>,
    next: Res<NextLevel>,
) {
    commands.trigger(OnPopModal);
    loaded_level.0 = level_list.0[next.0].clone();
    commands.remove_resource::<NextLevel>();
}

fn click_to_menu(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(OnGoTo(Screen::Title));
}
fn click_pop_modal(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(OnPopModal);
}
fn click_spawn_settings(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(OnNewModal(Modal::Settings));
}

fn trigger_menu_toggle_on_esc(
    _: Trigger<OnBack>,
    mut commands: Commands,
    screen: Res<State<Screen>>,
    settings: ResMut<Settings>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }
    if settings.modals.is_empty() {
        commands.trigger(OnNewModal(Modal::Main));
    } else {
        commands.trigger(OnPopModal);
    }
}

fn add_new_modal(
    trig: Trigger<OnNewModal>,
    screen: Res<State<Screen>>,
    mut commands: Commands,
    mut settings: ResMut<Settings>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }

    if settings.modals.is_empty() {
        commands.trigger(OnPauseToggle);
    }

    // despawn all previous modals
    commands.trigger(OnClearModals);
    let OnNewModal(modal) = trig.event();
    match modal {
        Modal::Main => commands.spawn(menu_modal()),
        Modal::Settings => commands.spawn(settings_modal()),
        Modal::LevelFinished => commands.spawn(level_finished_modal()),
    };

    settings.modals.push(modal.clone());
}

fn pop_modal(
    _: Trigger<OnPopModal>,
    mut commands: Commands,
    screen: Res<State<Screen>>,
    mut settings: ResMut<Settings>,
    menu_marker: Query<Entity, With<MenuModal>>,
    settings_marker: Query<Entity, With<SettingsModal>>,
    level_finished_marker: Query<Entity, With<LevelFinishedModal>>,
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
                commands.entity(menu).despawn();
            }
        }
        Modal::Settings => {
            if let Ok(settings) = settings_marker.single() {
                commands.entity(settings).despawn();
            }
        }
        Modal::LevelFinished => {
            if let Ok(gameover) = level_finished_marker.single() {
                commands.entity(gameover).despawn();
                commands.remove_resource::<NextLevel>();
            }
        }
    }

    // respawn next in the modal stack
    if let Some(modal) = settings.modals.last() {
        match modal {
            Modal::Main => {
                commands.spawn(menu_modal());
            }
            Modal::Settings => {
                commands.spawn(settings_modal());
            }
            _ => {}
        }
    }

    if settings.modals.is_empty() {
        commands.trigger(OnPauseToggle);
    }
}

fn clear_modals(
    _: Trigger<OnClearModals>,
    mut commands: Commands,
    settings: ResMut<Settings>,
    menu_marker: Query<Entity, With<MenuModal>>,
    settings_marker: Query<Entity, With<SettingsModal>>,
    gameover_marker: Query<Entity, With<LevelFinishedModal>>,
) {
    for m in &settings.modals {
        match m {
            Modal::Main => {
                if let Ok(menu) = menu_marker.single() {
                    commands.entity(menu).despawn();
                }
            }
            Modal::Settings => {
                if let Ok(settings) = settings_marker.single() {
                    commands.entity(settings).despawn();
                }
            }
            Modal::LevelFinished => {
                if let Ok(gameover) = gameover_marker.single() {
                    commands.entity(gameover).despawn();
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

fn level_finished_modal() -> impl Bundle {
    (
        StateScoped(Screen::Gameplay),
        LevelFinishedModal,
        ui_root("game over modal"),
        BackgroundColor(TRANSLUCENT),
        children![
            label("Level finished!"),
            btn_big("Main Menu", enter_gameplay_screen),
            btn_big("Next Level", click_to_next_level),
        ],
    )
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
