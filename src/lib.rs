#![allow(clippy::type_complexity, clippy::too_many_arguments)]
pub mod assets;
pub mod moving;
pub mod settings;
pub mod tiling;

pub use assets::*;
pub use moving::*;
pub use settings::*;
pub use tiling::*;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FinishReason {
    Lost,
    Won,
}

#[derive(Event, Debug)]
pub struct GameOver(FinishReason);
