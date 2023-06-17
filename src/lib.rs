pub mod assets;

pub use assets::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Finished,
}
