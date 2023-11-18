#![allow(clippy::type_complexity, clippy::too_many_arguments)]
pub mod assets;
pub mod audio;
pub mod moving;
pub mod settings;
pub mod tiling;
pub mod ui;

pub use assets::*;
pub use audio::*;
pub use moving::*;
pub use settings::*;
pub use tiling::*;
pub use ui::*;

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Setup,
    InGame,
    MainMenu,
    GameOverMenu,
    SettingsMenu,
    Paused,
}

impl States for AppState {}

#[derive(Event, Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameOver {
    //consider adding a score to this
    Lost,
    Won,
    Quit,
}
