use crate::prelude::*;
use bevy::prelude::*;
use bevy_seedling::{pool::Sampler, prelude::*};
use rand::prelude::*;

pub fn plugin(app: &mut App) {
    // app.add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
    //     .add_systems(OnEnter(Screen::Gameplay), start_or_resume_soundtrack);
}

// TODO: implement different music states
// good structure in this example: <https://github.com/bevyengine/bevy/blob/main/examples/audio/soundtrack.rs#L29>
fn start_or_resume_soundtrack(
    mut cmds: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    // boombox: Query<Entity, With<Boombox>>,
    mut music_query: Query<(&Sampler, &mut PlaybackSettings), With<Music>>,
) -> Result {
    if let Ok((player, mut instance)) = music_query.single_mut() {
        if !player.is_playing() {
            instance.play();
        }
    } else {
        let handle = *[&sources.bg_music].choose(&mut thread_rng()).unwrap();
        let vol = settings.sound.general * settings.sound.music;
        cmds.spawn(music(handle.clone(), vol));
    }

    Ok(())
}

fn stop_soundtrack(mut bg_music: Query<&mut PlaybackSettings, With<Music>>) {
    for mut s in bg_music.iter_mut() {
        s.pause();
    }
}
