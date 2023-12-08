use crate::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct CongratsMenu;

impl Menu for CongratsMenu {}

pub fn spawn_congrats_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((CongratsMenu, default_menu_backdrop()))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Congratulations! \nyou made a 2048 tile!",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 120.0,
                            color: Color::GOLD,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
}

pub fn trigger_congrats_menu(
    mut merges: EventReader<Merged>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    //2**11=2048
    if merges.read().any(|merged| merged.power() == 11) {
        next_state.set(AppState::CongratsMenu)
    }
}

pub fn return_to_game(
    mut next_state: ResMut<NextState<AppState>>,
    mut next_congratulated: ResMut<NextState<Congratulation>>,
    mouse_buttons: Res<Input<MouseButton>>,
    keyboard_buttons: Res<Input<KeyCode>>,
) {
    //if any input is registered from the user we get them back to the game
    if mouse_buttons.get_pressed().len() != 0 || keyboard_buttons.get_pressed().len() != 0 {
        next_state.set(AppState::InGame);
        next_congratulated.set(Congratulation::Congratulated);
    }
}
