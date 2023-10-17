#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use b2048::*;
use bevy::{
    app::AppExit,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(900.0, 900.0),
                title: "A 2048 clone by s0lst1ce".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_event::<GameOver>()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins((
            GameAssetsPlugin,
            UserSettingsPlugin,
            TilingPlugin,
            MovingPlugin,
            MusicPlugin,
        ))
        .add_systems(OnEnter(AppState::Setup), setup)
        //the systems responsible for running the game
        .add_systems(
            Update,
            (
                (
                    bevy::window::close_on_esc,
                    (game_over.after(spawn_tile),).run_if(in_state(AppState::InGame)),
                )
                    .after(setup),
                debug_info,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut spawn_tiles: EventWriter<SpawnTile>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tiling: ResMut<Tiling>,
    handles: Res<TileHandles>,
    board: Res<Board>,
    assets: Res<Assets<Image>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    //setting up tile sizes and descriptors for current window and asset dimensions
    let Ok(primary) = window.get_single() else {
        error!("no primary window");
        return;
    };
    resize_tiling(
        &mut tiling,
        &board,
        &assets.get(&handles.0[0]).unwrap().texture_descriptor,
        primary.width(),
        primary.height(),
    );

    //placing initial tiles
    spawn_tiles.send_batch([SpawnTile::default(); 2]);

    //starting the game
    next_state.set(AppState::InGame);

    info!("Game setup completed");
}

fn game_over(game_over: EventReader<GameOver>, mut exit: EventWriter<AppExit>) {
    if !game_over.is_empty() {
        info!("Game ended");
        exit.send(AppExit);
    }
}

fn debug_info(_pos: Query<&Position>) {}
