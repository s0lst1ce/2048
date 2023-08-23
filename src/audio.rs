use bevy::prelude::*;

#[derive(Debug, Component)]
struct BackgroundMusic;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/music.ogg"),
            ..default()
        },
        BackgroundMusic,
    ));
}

#[derive(Debug, Clone, Copy)]
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
