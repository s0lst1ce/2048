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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Setup,
    InGame,
    MainMenu,
    SettingsMenu,
    Paused,
    WonMenu,
    LostMenu,
    CongratsMenu,
}

#[derive(Event, Debug, Copy, Clone, PartialEq, Eq)]
pub enum FinishGame {
    GameOver,
    Quit,
}

#[derive(Debug, Resource, Clone, PartialEq, Eq, Default)]
pub struct Score(pub u32);

impl Score {
    pub fn has_won(&self) -> bool {
        self.0 > 0
    }
}

pub fn score_from_merge(mut score: ResMut<Score>, mut merges: EventReader<Merged>) {
    for merge in merges.read() {
        score.0 += merge.power()
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Congratulation {
    Congratulated,
    #[default]
    NotYet,
}
