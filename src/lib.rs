pub mod assets;

pub use assets::*;

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Setup,
    InGame,
}

impl States for AppState {
    type Iter = std::array::IntoIter<AppState, 3>;

    fn variants() -> Self::Iter {
        [AppState::Loading, AppState::Setup, AppState::InGame].into_iter()
    }
}
