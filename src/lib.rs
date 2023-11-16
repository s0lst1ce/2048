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
}

impl States for AppState {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FinishReason {
    Lost,
    Won,
}

#[derive(Event, Debug)]
pub struct GameOver(FinishReason);
