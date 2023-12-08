//todo make it so text in menus have a dynamic font size based on screen size

use crate::*;
use bevy::{app::AppExit, prelude::*};

mod congrats;
mod main_menu;
mod pause_menu;
mod won_menu;

use congrats::*;
use main_menu::*;
use pause_menu::*;
use won_menu::*;

#[derive(Debug)]
pub struct GameInterfacePlugin;

impl Plugin for GameInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TogglePause>()
            .add_state::<Congratulation>()
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_menu::<MainMenu>)
            .add_systems(OnEnter(AppState::Paused), spawn_pause_menu)
            .add_systems(OnExit(AppState::Paused), despawn_menu::<PauseMenu>)
            .add_systems(OnEnter(AppState::WonMenu), spawn_won_menu)
            .add_systems(OnExit(AppState::WonMenu), despawn_menu::<WonMenu>)
            .add_systems(OnEnter(AppState::CongratsMenu), spawn_congrats_menu)
            .add_systems(OnExit(AppState::CongratsMenu), despawn_menu::<CongratsMenu>)
            .add_systems(
                Update,
                (
                    start_game,
                    exit_app,
                    toggle_pause,
                    pause_with_keybind,
                    resume_game,
                    back_to_menu,
                    trigger_congrats_menu
                        .after(score_from_merge)
                        .run_if(in_state(Congratulation::NotYet)),
                    return_to_game.run_if(in_state(AppState::CongratsMenu)),
                ),
            );
    }
}

pub(crate) trait Menu: Component {}

#[derive(Debug, Component)]
pub(crate) struct ExitButton;

impl ExitButton {
    pub(crate) fn spawn(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        parent
            .spawn((
                ExitButton,
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
    }
}

pub(crate) const DEFAULT_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

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

#[derive(Debug, Event)]
pub struct TogglePause;

fn pause_with_keybind(
    keys: Res<Input<KeyCode>>,
    keybinds: Res<Keybinds>,
    mut toggle_pause: EventWriter<TogglePause>,
) {
    if keys.just_pressed(keybinds.pause_game) {
        toggle_pause.send(TogglePause);
    }
}

fn toggle_pause(
    mut toggle_pause: EventReader<TogglePause>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    //it is possible multiple toggle pause events were sent in the same tick, in which case we want to treat
    //them all correctly, not do a single toggle. Thus the number of events dictates the correct behavior.
    match toggle_pause.len() {
        //either we didn't toggle at all or;
        //we toggled an even number of times, so we effectively don't change anything
        i if i % 2 == 0 => (),
        _ => {
            if let Some(new_state) = match current_state.get() {
                AppState::InGame => Some(AppState::Paused),
                AppState::Paused => Some(AppState::InGame),
                _ => None,
            } {
                next_state.set(new_state)
            }
        }
    }
    //because we would read the same events once more in the next frame, if errors occur see explicit event ordering
    toggle_pause.clear();
}

pub(crate) fn default_menu_backdrop() -> NodeBundle {
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
    }
}

#[derive(Debug, Component)]
pub struct BackToMenuButton;

pub fn back_to_menu(
    query: Query<&Interaction, (Changed<Interaction>, With<BackToMenuButton>)>,
    mut game_over: EventWriter<FinishGame>,
) {
    if let Ok(Interaction::Pressed) = query.get_single() {
        game_over.send(FinishGame::Quit)
    }
}

impl BackToMenuButton {
    pub(crate) fn spawn(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        parent
            .spawn((
                BackToMenuButton,
                ButtonBundle {
                    style: DEFAULT_BUTTON_STYLE,
                    background_color: Color::hex("776e65").unwrap().into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Main Menu",
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
    }
}
