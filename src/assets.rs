use crate::*;
use bevy::{asset::LoadState, prelude::*};

#[derive(Debug, Resource, Default)]
pub struct TileHandles(pub Vec<Handle<Image>>);

impl TileHandles {
    const TILES: [u32; 14] = [
        0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
    ];
    fn paths() -> impl IntoIterator<Item = String> {
        Self::TILES.map(|int| format!("tiles/{int}.png"))
    }
}

#[derive(Debug, Resource, Default)]
pub struct TilesAtlas(pub Handle<TextureAtlas>);

fn load_assets(mut tile_handles: ResMut<TileHandles>, asset_server: Res<AssetServer>) {
    for path in TileHandles::paths() {
        let _handle = asset_server.load::<Image>(path);
    }
    let mut tiles = Vec::with_capacity(TileHandles::TILES.len());
    for path in TileHandles::paths() {
        let handle = asset_server.get_handle(&path).unwrap();
        tiles.push(handle);
    }

    tile_handles.0 = tiles;
}

fn check_assets(
    mut app_state: ResMut<NextState<AppState>>,
    tile_handles: ResMut<TileHandles>,
    asset_server: Res<AssetServer>,
) {
    if tile_handles
        .0
        .iter()
        .map(|handle| handle.id())
        .all(|id| asset_server.load_state(id) == LoadState::Loaded)
    {
        info!("Finished loading assets");
        app_state.set(AppState::Setup)
    }
}

fn post_load_setup(
    mut commands: Commands,
    tile_handles: Res<TileHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    //making tile assets available
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &tile_handles.0 {
        let Some(texture) = textures.get(handle) else {
            warn!("{:?} did not resolve to an `Image` asset", handle.path());
            continue;
        };

        texture_atlas_builder.add_texture(handle.id(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    commands.insert_resource(TilesAtlas(atlas_handle));
}

#[derive(Debug, Clone, Copy)]
pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileHandles>()
            .init_resource::<TilesAtlas>()
            .add_systems(OnEnter(AppState::Loading), load_assets)
            .add_systems(Update, check_assets.run_if(in_state(AppState::Loading)))
            .add_systems(OnExit(AppState::Loading), post_load_setup);
    }
}
