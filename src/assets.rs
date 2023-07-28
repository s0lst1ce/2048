use crate::*;
use bevy::{asset::LoadState, prelude::*};

#[derive(Debug, Resource, Default)]
pub struct TileHandles(pub Vec<Handle<Image>>);

#[derive(Debug, Resource, Default)]
pub struct TilesAtlas(pub Handle<TextureAtlas>);

fn load_assets(mut tiles_handles: ResMut<TileHandles>, asset_server: Res<AssetServer>) {
    tiles_handles.0 = asset_server
        .load_folder("tiles")
        .map_err(|err| {
            error!("missing `assets/tiles` folder");
            err
        })
        .unwrap()
        .iter()
        .map(|handle| handle.clone().typed())
        .collect();
}

fn check_assets(
    mut app_state: ResMut<NextState<AppState>>,
    tile_handles: ResMut<TileHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(tile_handles.0.iter().map(|handle| handle.id()))
    {
        info!("Finished loading assets");
        app_state.set(AppState::Setup)
    }
}

fn post_load_setup(
    mut commands: Commands,
    tile_handles: Res<TileHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    //making tile assets available
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &tile_handles.0 {
        let Some(texture) = textures.get(handle) else {
            warn!(
                "{:?} did not resolve to an `Image` asset",
                asset_server.get_handle_path(handle)
            );
            continue;
        };

        texture_atlas_builder.add_texture(handle.clone(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(TilesAtlas(atlas_handle));
}

#[derive(Debug)]
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
