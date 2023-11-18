use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct UserSettingsPlugin;

impl Plugin for UserSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Keybinds>();
    }
}

#[derive(Debug, Eq, PartialEq, Resource)]
pub struct Keybinds {
    pub move_left: KeyCode,
    pub move_up: KeyCode,
    pub move_right: KeyCode,
    pub move_down: KeyCode,
    pub pause_game: KeyCode,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            move_left: KeyCode::Left,
            move_up: KeyCode::Up,
            move_right: KeyCode::Right,
            move_down: KeyCode::Down,
            pause_game: KeyCode::Escape,
        }
    }
}
