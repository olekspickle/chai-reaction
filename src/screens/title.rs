use super::*;

/// This plugin is responsible for the game menu
/// The menu is only drawn during the State [`Screen::Title`] and is removed when that state is exited
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), setup_menu);
}

fn setup_menu(mut commands: Commands, sources: Res<AudioSources>, settings: Res<Settings>, level_list: Res<LevelList>, level_configs: Res<Assets<LevelConfig>>, editor_mode: Res<EditorMode>, mut next_screen: ResMut<NextState<Screen>>, cfg: Res<Config>) {
    if editor_mode.0 {
        next_screen.set(Screen::Gameplay);
        return
    }
    commands.insert_resource(avian2d::prelude::Gravity(Vec2::NEG_Y * 9.81 * cfg.physics.gravity));

    let levels: Vec<_> = level_list.0.iter().enumerate().map(|(i, h)| {
        let name = level_configs.get(h).unwrap().name.clone();
        btn_big(name, move |_: Trigger<OnPress>, mut cmds: Commands, level_list: Res<LevelList>, mut next_screen: ResMut<NextState<Screen>>,| {
            next_screen.set(Screen::Gameplay);
            cmds.insert_resource(LoadedLevel(level_list.0[i].clone()));
        })
    }).collect();
    commands.spawn((
        StateScoped(Screen::Title),
        ui_root("Title"),
        // Crutch until we can use #cfg in children![] macro
        // https://github.com/bevyengine/bevy/issues/18953
    )).with_children(|parent| {
        parent.spawn(BackgroundColor(TRANSLUCENT));
        for level in levels {
            parent.spawn(level);
        }
        parent.spawn(btn_big("Credits", to::credits));
        parent.spawn(btn_big("Settings", to::settings));
        #[cfg(not(target_family = "wasm"))]
        parent.spawn(
            btn_big("Exit", exit_app)
        );
    });
    commands.spawn((
        StateScoped(Screen::Title),
        music_looping(sources.menu.clone(), settings.music()),
    ));
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
