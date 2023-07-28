use b2048::*;
use bevy::{app::AppExit, prelude::*};
use rand::{seq::IteratorRandom, thread_rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(GameAssetsPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_event::<GameOver>()
        .add_event::<SpawnTile>()
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(OnEnter(AppState::Setup), setup_board)
        .add_systems(OnEnter(AppState::Setup), setup.after(setup_board))
        //the systems responsible for running the game
        .add_systems(
            Update,
            (spawn_tile, game_over).run_if(in_state(AppState::InGame)),
        )
        .run();
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

#[derive(Resource, Debug)]
struct Board {
    columns: usize,
    rows: usize,
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
    TileKind(1)
}

fn spawn_tile(
    mut commands: Commands,
    tiles: Query<&Position, With<Tile>>,
    board: Res<Board>,
    new_tiles: EventReader<SpawnTile>,
    mut game_over: EventWriter<GameOver>,
    tiles_atlas: Res<TilesAtlas>,
) {
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
                        sprite: TextureAtlasSprite::new(kind.index()),
                        texture_atlas: atlas_handle.clone(),
                        ..default()
                    },
                    _tile: Tile,
                }
            }),
    );
}

fn setup_board(mut commands: Commands) {
    //board description
    let board = Board {
        columns: 4,
        rows: 4,
    };
    commands.insert_resource(board);
}

fn setup(
    mut commands: Commands,
    mut spawn_tiles: EventWriter<SpawnTile>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    commands.spawn(Camera2dBundle::default());

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
