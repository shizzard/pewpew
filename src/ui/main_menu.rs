use bevy::prelude::*;

use crate::game::state::State as GameState;
use crate::game::transition::Event;
use crate::game::SystemSet;

const ROOT_NODE_COLOR: Color = Color::BLACK;
const BUTTON_NORMAL_COLOR: Color = Color::BLACK;
const PLAY_BUTTON_HOVER_COLOR: Color = Color::MAROON;
const QUIT_BUTTON_HOVER_COLOR: Color = Color::MAROON;

const LOGO_TEXT: &'static str = "PEW-PEW!";
const LOGO_TEXT_SIZE: f32 = 100.;
const PLAY_BUTTON_TEXT: &'static str = "PLAY";
const PLAY_BUTTON_TEXT_SIZE: f32 = 50.;
const QUIT_BUTTON_TEXT: &'static str = "QUIT";
const QUIT_BUTTON_TEXT_SIZE: f32 = 50.;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Component, Debug)]
pub struct PlayButton;

#[derive(Component, Debug)]
pub struct QuitButton;

pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu)
            .add_systems(
                Update,
                (play_button_handler, quit_button_handler).in_set(SystemSet::MainMenu),
            );
    }
}

fn spawn_menu(mut cmd: Commands) {
    cmd.spawn(root_node())
        .with_children(main_menu_wrapper)
        .insert(Tag);
}

fn root_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect {
                left: Val::Percent(20.0),
                right: Val::Percent(20.0),
                top: Val::Percent(20.0),
                bottom: Val::Percent(20.0),
            },
            ..default()
        },
        background_color: ROOT_NODE_COLOR.into(),
        ..default()
    }
}

fn main_menu_wrapper(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .with_children(game_logo)
    .with_children(play_button)
    .with_children(quit_button);
}

fn game_logo(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .with_children(game_logo_text);
}

fn game_logo_text(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section(LOGO_TEXT, TextStyle {
        font_size: LOGO_TEXT_SIZE,
        color: Color::WHITE,
        ..default()
    }));
}

fn play_button(root: &mut ChildBuilder) {
    root.spawn(ButtonBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(20.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Percent(5.0)),
            border: UiRect::all(Val::Px(10.)),
            ..default()
        },
        background_color: BUTTON_NORMAL_COLOR.into(),
        border_color: PLAY_BUTTON_HOVER_COLOR.into(),
        ..default()
    })
    .insert(PlayButton)
    .with_children(play_button_text);
}

fn play_button_text(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section(PLAY_BUTTON_TEXT, TextStyle {
        font_size: PLAY_BUTTON_TEXT_SIZE,
        color: Color::WHITE,
        ..default()
    }));
}

fn quit_button(root: &mut ChildBuilder) {
    root.spawn(ButtonBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(20.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Percent(5.0)),
            border: UiRect::all(Val::Px(10.)),
            ..default()
        },
        background_color: BUTTON_NORMAL_COLOR.into(),
        border_color: QUIT_BUTTON_HOVER_COLOR.into(),
        ..default()
    })
    .insert(QuitButton)
    .with_children(quit_button_text);
}

fn quit_button_text(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section(QUIT_BUTTON_TEXT, TextStyle {
        font_size: QUIT_BUTTON_TEXT_SIZE,
        color: Color::WHITE,
        ..default()
    }));
}

fn despawn_menu(mut cmd: Commands, query: Query<Entity, With<Tag>>) {
    let Ok(menu) = query.get_single() else {
        return;
    };
    cmd.entity(menu).despawn_recursive();
}

fn play_button_handler(
    mut evw_transition: EventWriter<Event>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                evw_transition.send(Event::Play);
            }
            Interaction::Hovered => {
                *color = PLAY_BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_NORMAL_COLOR.into();
            }
        }
    }
}

fn quit_button_handler(
    mut evw_transition: EventWriter<Event>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                evw_transition.send(Event::Quit);
            }
            Interaction::Hovered => {
                *color = QUIT_BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_NORMAL_COLOR.into();
            }
        };
    }
}
