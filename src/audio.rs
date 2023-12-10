use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::{loading::LoadingAssets, states::AppState};

#[derive(Debug, Default, Resource)]
struct MusicAssets {
    tracks: Vec<Handle<AudioSource>>,
}

fn load_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let mut tracks = vec![
        asset_server.load::<AudioSource>("music/Alpha Hydrae - La Pêche.mp3"),
        asset_server.load::<AudioSource>("music/Zoliborz - To Balagopalan Ganapathy.mp3"),
        asset_server.load::<AudioSource>("music/Alpha Hydrae - To be like a chased rabbit.mp3"),
        asset_server.load::<AudioSource>("music/Cathedral Of Chemical Equilibrium - One.mp3"),
        asset_server.load::<AudioSource>("music/Monplaisir - This is not a joke.mp3"),
    ];

    tracks.shuffle(&mut rand::thread_rng());

    for track in tracks.iter() {
        loading_assets.add(track.clone());
    }

    commands.insert_resource(MusicAssets { tracks });
}

#[derive(Component, Debug, Default)]
struct MusicPlayer {
    track: Option<usize>,
}

fn setup_music(mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Once,
                ..Default::default()
            },
            ..Default::default()
        },
        MusicPlayer::default(),
    ));
}

fn play_music(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Handle<AudioSource>,
        &mut MusicPlayer,
        Option<&AudioSink>,
    )>,
    music_assets: Res<MusicAssets>,
) {
    if music_assets.tracks.is_empty() {
        return;
    }

    for (e, mut audio, mut player, sink) in query.iter_mut() {
        if let Some(sink) = sink {
            if !sink.empty() {
                continue;
            }
        }
        // Either there's nothing playing or it was never started in the first place

        let track_no = if let Some(track) = player.track {
            (track + 1) % music_assets.tracks.len()
        } else {
            0
        };

        player.track = Some(track_no);

        *audio = music_assets.tracks[track_no].clone();
        commands.entity(e).remove::<AudioSink>();
    }
}

fn handle_volume_change(query: Query<&AudioSink>, volume_settings: Res<VolumeSettings>) {
    if volume_settings.is_changed() {
        for sink in query.iter() {
            sink.set_volume(volume_settings.volume);
        }
    }
}

fn handle_new_sinks(
    query: Query<&AudioSink, Added<AudioSink>>,
    volume_settings: Res<VolumeSettings>,
) {
    for sink in query.iter() {
        sink.set_volume(volume_settings.volume);
    }
}

#[derive(Debug, Resource)]
pub struct VolumeSettings {
    pub volume: f32,
}

impl Default for VolumeSettings {
    fn default() -> Self {
        VolumeSettings { volume: 0.5 }
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VolumeSettings>()
            .add_systems(Startup, load_music)
            .add_systems(OnExit(AppState::Loading), setup_music)
            .add_systems(Update, (handle_volume_change, handle_new_sinks, play_music));
    }
}
