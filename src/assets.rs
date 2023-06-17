use crate::*;
use bevy::{asset::LoadState, prelude::*};

impl States for AppState {
    type Iter = std::array::IntoIter<AppState, 2>;

    fn variants() -> Self::Iter {
        [AppState::Loading, AppState::Finished].into_iter()
    }
}

#[derive(Debug, Resource, Default)]
pub struct TileHandles(pub Vec<HandleUntyped>);

fn load_assets(asset_server: Res<AssetServer>) {
    asset_server
        .load_folder("tiles")
        .map_err(|err| {
            error!("missing `assets/tiles` folder");
            err
        })
        .unwrap();
}

fn check_assets(
    mut next_state: ResMut<NextState<AppState>>,
    tile_handles: ResMut<TileHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(tile_handles.0.iter().map(|handle| handle.id()))
    {
        info!("Finished loading assets");
        next_state.set(AppState::Finished)
    }
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileHandles>()
            .add_state::<AppState>()
            .add_system(load_assets.in_schedule(OnEnter(AppState::Loading)))
            .add_system(check_assets.in_set(OnUpdate(AppState::Loading)));
    }
}
