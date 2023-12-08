use crate::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct PauseMenu;

impl Menu for PauseMenu {}

#[derive(Debug, Component)]
pub struct ResumeButton;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((PauseMenu, default_menu_backdrop()))
        .with_children(|parent| {
            //paused text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Paused",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 128.0,
                            color: Color::BLACK,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });

            //resume text
            parent
                .spawn((
                    ResumeButton,
                    ButtonBundle {
                        style: DEFAULT_BUTTON_STYLE,
                        background_color: Color::BLUE.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Resume",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            //main menu
            BackToMenuButton::spawn(parent, &asset_server);

            ExitButton::spawn(parent, &asset_server);
        });
}

pub fn resume_game(
    query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    mut toggle_pause: EventWriter<TogglePause>,
) {
    //we don't need to check if we're in the right AppState because we're already checking for a button interaction
    if let Ok(Interaction::Pressed) = query.get_single() {
        toggle_pause.send(TogglePause)
    }
}
