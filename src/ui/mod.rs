use crate::*;
use bevy::{app::AppExit, prelude::*};

mod main_menu;
mod pause_menu;

use main_menu::*;
use pause_menu::*;

#[derive(Debug)]
pub struct GameInterfacePlugin;

impl Plugin for GameInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TogglePause>()
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_menu::<MainMenu>)
            .add_systems(
                Update,
                (
                    start_game,
                    exit_app,
                    toggle_pause,
                    pause_with_keybind,
                    resume_game,
                    back_to_menu,
                ),
            )
            .add_systems(OnEnter(AppState::Paused), spawn_pause_menu)
            .add_systems(OnExit(AppState::Paused), despawn_menu::<PauseMenu>);
    }
}

pub(crate) trait Menu: Component {}

#[derive(Debug, Component)]
pub(crate) struct ExitButton;

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

pub(crate) fn spawn_exit_button(parent: &mut ChildBuilder, asset_server: Res<AssetServer>) {
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
