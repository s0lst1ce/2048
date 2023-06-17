use b2048::*;
use bevy::prelude::*;
use rand::{seq::IteratorRandom, thread_rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameAssetsPlugin)
        .add_system(bevy::window::close_on_esc)
        .insert_resource(ClearColor(Color::WHITE))
        .add_event::<GameOver>()
        .add_system(setup_board.in_schedule(OnEnter(AppState::Finished)))
        .add_startup_system(setup_board)
        .add_system(place_new_tile)
        .run();
}

#[derive(Component, Debug)]
struct Tile;

#[derive(Bundle)]
struct TileBundle {
    position: Position,
    kind: TileKind,
    sprite: SpriteBundle,
}

impl TileBundle {
    fn new(position: Position, kind: TileKind, tile_texture_handle: TileTextureHandle) -> Self {
        Self {
            position,
            kind,
            sprite: SpriteBundle {
                texture: tile_texture_handle.0,
                transform: Transform {
                    translation: Vec3 {
                        x: 10.0,
                        y: 10.0,
                        z: 1.0,
                    },
                    scale: Vec3::default(),
                    ..default()
                },
                ..default()
            },
        }
    }
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FinishReason {
    Lost,
    Won,
}

#[derive(Debug)]
struct GameOver(FinishReason);

fn new_tile_value() -> TileKind {
    TileKind(1)
}

#[derive(Debug, Resource, Clone)]
struct TileTextureHandle(Handle<Image>);

fn place_new_tile(
    mut commands: Commands,
    tiles: Query<&Position, With<Tile>>,
    board: Res<Board>,
    mut game_over: EventWriter<GameOver>,
    tile_texture_handle: Res<TileTextureHandle>,
) {
    //finding free tiles
    let mut occupied = Vec::with_capacity(board.columns * board.rows);

    tiles.iter().for_each(|pos| occupied.push(pos));

    let mut rng = thread_rng();
    //choosing where to create the new tile if possible
    if let Some(pos) = (0..(board.columns * board.rows))
        .filter(|pos| !occupied.contains(&&Position(*pos)))
        .choose(&mut rng)
    {
        commands.spawn(TileBundle::new(
            Position(pos),
            new_tile_value(),
            tile_texture_handle.clone(),
        ));
    } else {
        game_over.send(GameOver(FinishReason::Lost));
    }
}

fn setup_board(
    mut commands: Commands,
    tile_handles: Res<TileHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    //making tile assets available
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &tile_handles.0 {
        let handle = handle.typed_weak();
        let Some(texture) = textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset", asset_server.get_handle_path(handle));
            continue;
        };

        texture_atlas_builder.add_texture(handle, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_texture = texture_atlas.texture.clone();
    let _atlas_handle = texture_atlases.add(texture_atlas);
    let tile_texture_handle = TileTextureHandle(texture_atlas_texture);
    commands.insert_resource(tile_texture_handle.clone());

    //board description
    let board = Board {
        columns: 4,
        rows: 4,
    };

    //placing initial tiles
    let mut rng = thread_rng();
    (0..(board.columns * board.rows))
        .choose_multiple(&mut rng, 2)
        .into_iter()
        .map(Position)
        .for_each(|pos| {
            commands.spawn(TileBundle::new(
                pos,
                new_tile_value(),
                tile_texture_handle.clone(),
            ));
        });

    commands.insert_resource(board);

    info!("Game setup completed");
}
