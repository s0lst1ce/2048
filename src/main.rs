#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use b2048::*;
use bevy::{
    app::AppExit,
    prelude::*,
    render::render_resource::TextureDescriptor,
    window::{PrimaryWindow, WindowResized, WindowResolution},
};
use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(900.0, 900.0),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_plugins(GameAssetsPlugin)
        .add_event::<GameOver>()
        .add_event::<SpawnTile>()
        .insert_resource(ClearColor(Color::WHITE))
        .init_resource::<Tiling>()
        .init_resource::<Board>()
        .add_systems(OnEnter(AppState::Setup), setup)
        //the systems responsible for running the game
        .add_systems(
            Update,
            (
                (
                    bevy::window::close_on_esc,
                    (
                        resize_tiles.before(spawn_tile),
                        spawn_tile,
                        game_over.after(spawn_tile),
                        move_tiles,
                    )
                        .run_if(in_state(AppState::InGame)),
                )
                    .after(setup),
                debug_info,
            ),
        )
        .run();
}

#[derive(Debug, Resource, PartialEq, Clone)]
struct Tiling {
    /// Width of a tile as shown on screen
    width: f32,
    /// Height of a tile as shown on screen
    height: f32,
    /// Horizontal spacing between two tiles
    horizontal_spacing: f32,
    /// Vertical spacing between two tiles
    vertical_spacing: f32,
    /// Scaling
    horizontal_scale: f32,
    vertical_scale: f32,
}
impl Default for Tiling {
    fn default() -> Self {
        Tiling {
            horizontal_scale: 1.0,
            vertical_scale: 1.0,
            width: 0.0,
            height: 0.0,
            horizontal_spacing: 0.0,
            vertical_spacing: 0.0,
        }
    }
}

#[derive(Component, Debug)]
struct Tile;

#[derive(Bundle)]
struct TileBundle {
    position: Position,
    kind: TileKind,
    sprite: SpriteSheetBundle,
    _tile: Tile,
}

///The position of the tile of the board
///
/// Underlying implementation currently makes it an index. For a board of size (4x4), Position(5) is first column second row.
#[derive(Component, Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Position(usize);

impl Position {
    fn to_translation(self, tiling: &Tiling, board: &Board, window: &Window) -> Vec3 {
        let mut translation = Vec3::ZERO;
        let row = self.0 / board.columns; //for integers `/` is a floor division
        let col = self.0 % board.rows;

        // we set the abscissa to as many times the width of a tile as there are tiles before (on the left, ie the row number)
        // to this we add the spacing, but multiply it by one less because we don't account for the space *after* the tile we're placing
        // this would otherwise place the tile too far and cause issues on the right-end side of the board
        // then we finally add half the size of the tile itself because the origin is the center of the tile
        // because the origin of the window is also its center, we offset it similarly
        translation.x = col as f32 * tiling.width
            + col.saturating_sub(1) as f32 * tiling.horizontal_spacing
            - (window.width() - tiling.width) / 2.0;

        // same as above but for the ordinates
        translation.y = -(row as f32 * tiling.height)
            - row.saturating_sub(1) as f32 * tiling.vertical_spacing
            + (window.height() - tiling.height) / 2.0;

        translation
    }
}

#[derive(Resource, Debug)]
struct Board {
    columns: usize,
    rows: usize,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            columns: 4,
            rows: 4,
        }
    }
}

///Represents the value of a tile.
///
/// This is to be understood at the power of two it corresponds to. For example `four` is `TileKind::Two` because 2²=4.
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
struct TileKind(u32);

impl TileKind {
    fn from_value(value: u32) -> Self {
        TileKind(value.ilog2())
    }

    fn index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FinishReason {
    Lost,
    Won,
}

#[derive(Event, Debug)]
struct GameOver(FinishReason);

#[derive(Event, Debug, Clone, Copy)]
struct SpawnTile;

fn new_tile_value() -> TileKind {
    let mut rng = thread_rng();
    let pow = [(1, 7), (2, 3)]
        .choose_weighted(&mut rng, |(_, weight)| *weight)
        .unwrap();

    TileKind(pow.0)
}

fn spawn_tile(
    mut commands: Commands,
    tiles: Query<&Position, With<Tile>>,
    board: Res<Board>,
    tiling: Res<Tiling>,
    new_tiles: EventReader<SpawnTile>,
    mut game_over: EventWriter<GameOver>,
    tiles_atlas: Res<TilesAtlas>,
) {
    if new_tiles.is_empty() {
        return;
    }
    //finding free tiles
    let mut occupied = Vec::with_capacity(board.columns * board.rows);

    tiles.iter().for_each(|pos| occupied.push(pos));

    let amount = new_tiles.len();
    //we make sure there's enough space to spawn all the tiles necessary
    if occupied.capacity() - occupied.len() < amount {
        game_over.send(GameOver(FinishReason::Lost));
        return;
    }

    let mut rng = thread_rng();
    //choosing where to create the new tile if possible
    let atlas_handle = tiles_atlas.0.clone(); //fixme, there's one too many clone
    let (abscissa, ordinate) = (tiling.horizontal_scale, tiling.vertical_scale);
    commands.spawn_batch(
        (0..(board.columns * board.rows))
            .filter(|pos| !occupied.contains(&&Position(*pos)))
            .choose_multiple(&mut rng, amount)
            .into_iter()
            .map(move |pos| {
                let kind = new_tile_value();
                TileBundle {
                    position: Position(pos),
                    kind,
                    sprite: SpriteSheetBundle {
                        transform: Transform {
                            scale: Vec3::new(abscissa, ordinate, 1.0),
                            ..default()
                        },
                        sprite: TextureAtlasSprite::new(kind.index()),
                        texture_atlas: atlas_handle.clone(),
                        ..default()
                    },
                    _tile: Tile,
                }
            }),
    );
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
    spawn_tiles.send_batch([SpawnTile; 2]);

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

fn move_tiles(
    mut tiles: Query<(&Position, &mut Transform), (With<Tile>, Changed<Position>)>,
    window: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    tiling: Res<Tiling>,
) {
    let Ok(primary) = window.get_single() else {
        error!("No window exists!");
        return;
    };

    for (pos, mut transform) in &mut tiles {
        transform.translation = pos.to_translation(&tiling, &board, primary)
    }
}

fn resize_tiles(
    mut tiles: Query<&mut Transform, With<Tile>>,
    mut resize: EventReader<WindowResized>,
    mut tiling: ResMut<Tiling>,
    handles: Res<TileHandles>,
    board: Res<Board>,
    assets: Res<Assets<Image>>,
) {
    let Some(window_dims) = resize.iter().last() else {
        return;
    };

    resize_tiling(
        &mut tiling,
        &board,
        &assets.get(&handles.0[0]).unwrap().texture_descriptor,
        window_dims.width,
        window_dims.height,
    );

    for mut transform in &mut tiles {
        transform.scale = Vec3::new(tiling.horizontal_scale, tiling.vertical_scale, 0.0);
    }
}

fn resize_tiling(
    tiling: &mut Tiling,
    board: &Board,
    tile_descriptor: &TextureDescriptor,
    win_width: f32,
    win_height: f32,
) {
    //retrieving tile images dimensions
    let dims = tile_descriptor.size;

    let horizontal_scale = win_width / (board.columns as u32 * dims.width) as f32;
    let vertical_scale = win_height / (board.rows as u32 * dims.height) as f32;

    //update the relevant fields
    tiling.width = dims.width as f32 * horizontal_scale;
    tiling.height = dims.height as f32 * vertical_scale;
    tiling.horizontal_scale = horizontal_scale;
    tiling.vertical_scale = vertical_scale;
}

fn debug_info(_pos: Query<&Position>) {}
