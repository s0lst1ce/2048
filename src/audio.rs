use bevy::{audio::PlaybackMode, prelude::*};

use crate::AppState;

#[derive(Debug, Component)]
struct BackgroundMusic;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/music.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        BackgroundMusic,
    ));
}

#[derive(Debug, Clone, Copy)]
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup);
    }
}
