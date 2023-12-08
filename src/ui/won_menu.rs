use crate::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct WonMenu;

impl Menu for WonMenu {}

pub fn spawn_won_menu(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn((WonMenu, default_menu_backdrop()))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "🎉 Congratulations! 🎉",
                            TextStyle {
                                font: font.clone(),
                                font_size: 90.0,
                                color: Color::GOLD,
                            },
                        ),
                        TextSection::new(
                            format!("You won with a score of {}!", score.0),
                            TextStyle {
                                font,
                                font_size: 60.0,
                                color: Color::GOLD,
                            },
                        ),
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });

            BackToMenuButton::spawn(parent, &asset_server);
            ExitButton::spawn(parent, &asset_server);
        });
}
