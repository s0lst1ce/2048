use crate::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct LostMenu;

impl Menu for LostMenu {}

pub fn spawn_lost_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn((LostMenu, default_menu_backdrop()))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "You lost...\nBetter luck next time!",
                        TextStyle {
                            font: font.clone(),
                            font_size: 90.0,
                            color: Color::VIOLET,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });

            BackToMenuButton::spawn(parent, &asset_server);
        });
}
