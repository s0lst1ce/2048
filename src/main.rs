#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use b2048::*;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;

fn main() {
    //to print debug messages to browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

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
        .add_event::<FinishGame>()
        .insert_resource(ClearColor(Color::WHITE))
        .init_resource::<Score>()
        .add_plugins((
            GameAssetsPlugin,
            UserSettingsPlugin,
            TilingPlugin,
            MovingPlugin,
            MusicPlugin,
            GameInterfacePlugin,
        ))
        .add_systems(OnEnter(AppState::Setup), (game_setup, reset_score))
        .add_systems(OnExit(AppState::Loading), app_setup)
        //the systems responsible for running the game
        .add_systems(
            Update,
            (
                ((game_over.after(spawn_tile),),).after(game_setup),
                detect_stale_board.after(apply_move),
                score_from_merge,
            ),
        )
        .run();
}

fn app_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn game_setup(
    mut spawn_tiles: EventWriter<SpawnTile>,
    mut app_state: ResMut<NextState<AppState>>,
    mut congrats_state: ResMut<NextState<Congratulation>>,
    mut tiling: ResMut<Tiling>,
    handles: Res<TileHandles>,
    board: Res<Board>,
    assets: Res<Assets<Image>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
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

    //we haven't congratulated yet since no 2048 was created
    congrats_state.set(Congratulation::NotYet);

    //starting the game
    app_state.set(AppState::InGame);

    info!("Game setup completed");
}

fn game_over(
    mut commands: Commands,
    mut game_over: EventReader<FinishGame>,
    mut app_state: ResMut<NextState<AppState>>,
    tiles_query: Query<Entity, With<Tile>>,
    score: Res<Score>,
) {
    game_over.read().enumerate().for_each(|(i, reason)| {
        if i != 0 {
            error!("Multiple `GameOver` events in the same tick");
        }
        for entity in tiles_query.iter() {
            commands.entity(entity).despawn()
        }

        app_state.set(match reason {
            FinishGame::Quit => AppState::MainMenu,
            FinishGame::GameOver => {
                if score.has_won() {
                    AppState::WonMenu
                } else {
                    AppState::LostMenu
                }
            }
        })
    });
}

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0
}
