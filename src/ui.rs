use crate::*;

use bevy::{app::AppExit, prelude::*, ui};

#[derive(Debug)]
pub struct GameInterfacePlugin;

impl Plugin for GameInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_menu::<MainMenu>)
            .add_systems(Update, (start_game, exit_app));
    }
}

trait Menu: Component {}

#[derive(Debug, Component)]
struct MainMenu;

impl Menu for MainMenu {}

#[derive(Debug, Component)]
struct StartButton;

#[derive(Debug, Component)]
struct ExitButton;

const DEFAULT_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tiles: Res<TileHandles>,
) {
    commands
        .spawn((
            NodeBundle {
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.8).into(),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            //title
            parent
                .spawn(NodeBundle {
                    background_color: TRANSPARENT.into(),
                    style: Style {
                        width: Val::Px(900.0),
                        height: Val::Px(200.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    //the powers of two denoting the tiles used to write "2048" on screen
                    [1, 0, 2, 3].iter().for_each(|power| {
                        parent.spawn(ImageBundle {
                            background_color: Color::WHITE.into(),
                            image: UiImage {
                                texture: tiles.0.get(*power as usize).unwrap().clone(),
                                ..default()
                            },
                            ..default()
                        });
                    });
                });
            //start game button
            parent
                .spawn((
                    StartButton,
                    ButtonBundle {
                        background_color: Color::hex("776e65").unwrap().into(),
                        style: DEFAULT_BUTTON_STYLE,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
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
            //quit button
            parent
                .spawn((
                    StartButton,
                    ButtonBundle {
                        background_color: Color::hex("f65e3b").unwrap().into(),
                        style: DEFAULT_BUTTON_STYLE,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Exit",
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
        });
}

fn start_game(
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(Interaction::Pressed) = query.get_single() {
        app_state.set(AppState::Setup)
    }
}

fn exit_app(
    query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok(Interaction::Pressed) = query.get_single() {
        exit.send(AppExit)
    }
}

fn despawn_menu<M: Menu>(mut commands: Commands, main_menu: Query<Entity, With<M>>) {
    if let Ok(menu) = main_menu.get_single() {
        commands.entity(menu).despawn_recursive()
    }
}
