use bevy::prelude::*;

use crate::state::GameState;
use crate::transition::GameStateTransitionEvent;
use crate::GameSystemSet;

const ROOT_NODE_COLOR: Color = Color::rgb(0., 0., 0.);
const BUTTON_NORMAL_COLOR: Color = Color::BLACK;
const STATS_PLACEHOLDER_TEXT_COLOR: Color = Color::MAROON;
const MAIN_MENU_BUTTON_HOVER_COLOR: Color = Color::MAROON;

const GAME_OVER_TEXT: &str = "GAME OVER!";
const GAME_OVER_TEXT_SIZE: f32 = 100.;
const STATS_PLACEHOLDER_TEXT: &str = "GAME STATS PLACEHOLDER";
const STATS_PLACEHOLDER_TEXT_SIZE: f32 = 20.;
const MAIN_MENU_BUTTON_TEXT: &str = "MAIN MENU";
const MAIN_MENU_BUTTON_TEXT_SIZE: f32 = 50.;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Component, Debug)]
pub struct MainMenuButton;

pub struct GameOverUIPlugin;

impl Plugin for GameOverUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_menu)
            .add_systems(OnExit(GameState::GameOver), despawn_menu)
            .add_systems(
                Update,
                main_menu_button_handler.in_set(GameSystemSet::GameOver),
            );
    }
}

fn spawn_menu(mut cmd: Commands) {
    cmd.spawn(root_node())
        .with_children(main_menu_wrapper)
        .insert(Tag);
}

fn despawn_menu(mut cmd: Commands, query: Query<Entity, With<Tag>>) {
    let Ok(menu) = query.get_single() else {
        return;
    };
    cmd.entity(menu).despawn_recursive();
}

fn main_menu_button_handler(
    mut evw_transition: EventWriter<GameStateTransitionEvent>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                evw_transition.send(GameStateTransitionEvent::QuitEncounter);
            }
            Interaction::Hovered => {
                *color = MAIN_MENU_BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_NORMAL_COLOR.into();
            }
        };
    }
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
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: ROOT_NODE_COLOR.into(),
        z_index: ZIndex::Global(5),
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
    .with_children(game_over)
    .with_children(stats_placeholder)
    .with_children(main_menu_button);
}

fn game_over(root: &mut ChildBuilder) {
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
    .with_children(game_over_text);
}

fn game_over_text(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section(GAME_OVER_TEXT, TextStyle {
        font_size: GAME_OVER_TEXT_SIZE,
        color: Color::WHITE,
        ..default()
    }));
}

fn stats_placeholder(root: &mut ChildBuilder) {
    root.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: STATS_PLACEHOLDER_TEXT.into(),
                style: TextStyle {
                    font_size: STATS_PLACEHOLDER_TEXT_SIZE,
                    color: STATS_PLACEHOLDER_TEXT_COLOR,
                    ..default()
                },
            }],
            ..default()
        },
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
        ..default()
    });
}

fn main_menu_button(root: &mut ChildBuilder) {
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
        border_color: MAIN_MENU_BUTTON_HOVER_COLOR.into(),
        ..default()
    })
    .insert(MainMenuButton)
    .with_children(main_menu_button_text);
}

fn main_menu_button_text(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section(MAIN_MENU_BUTTON_TEXT, TextStyle {
        font_size: MAIN_MENU_BUTTON_TEXT_SIZE,
        color: Color::WHITE,
        ..default()
    }));
}
