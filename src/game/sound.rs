use crate::prelude::*;
use bevy::prelude::*;
use bevy_seedling::{pool::Sampler, prelude::*};
use rand::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
        .add_systems(OnEnter(Screen::Gameplay), start_or_resume_soundtrack);
}

fn start_or_resume_soundtrack(
    mut cmds: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    mut music_query: Query<(&Sampler, &mut PlaybackSettings), With<Music>>,
) -> Result {
    if let Ok((player, mut instance)) = music_query.single_mut() {
        if !player.is_playing() {
            instance.play();
        }
    } else {
        let handle = *[&sources.bg_music].choose(&mut thread_rng()).unwrap();
        cmds.spawn(music_looping(handle.clone(), settings.music()));
    }

    Ok(())
}

fn stop_soundtrack(mut bg_music: Query<&mut PlaybackSettings, With<Music>>) {
    for mut s in bg_music.iter_mut() {
        s.pause();
    }
}
