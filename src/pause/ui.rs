use bevy::prelude::*;

use super::transition::EncounterPauseStateTransitionEvent;
use super::transition::PauseState;
use crate::transition::GameStateTransitionEvent;
use crate::GameSystemSet;

const ROOT_NODE_COLOR: Color = Color::rgba(0., 0., 0., 0.9);
const BUTTON_NORMAL_COLOR: Color = Color::BLACK;
const CONTINUE_BUTTON_HOVER_COLOR: Color = Color::MAROON;
const MAIN_MENU_BUTTON_HOVER_COLOR: Color = Color::MAROON;

const LOGO_TEXT: &str = "PEW-PEW!";
const LOGO_TEXT_SIZE: f32 = 100.;
const CONTINUE_BUTTON_TEXT: &str = "CONTINUE";
const CONTINUE_BUTTON_TEXT_SIZE: f32 = 50.;
const MAIN_MENU_BUTTON_TEXT: &str = "MAIN MENU";
const MAIN_MENU_BUTTON_TEXT_SIZE: f32 = 50.;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Component, Debug)]
pub struct ContinueButton;

#[derive(Component, Debug)]
pub struct MainMenuButton;

pub struct PauseMenuUIPlugin;

impl Plugin for PauseMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PauseState::Pause), spawn_menu)
            .add_systems(OnEnter(PauseState::Running), despawn_menu)
            .add_systems(
                Update,
                (continue_button_handler, main_menu_button_handler)
                    .in_set(GameSystemSet::Encounter),
            )
            .add_systems(
                Update,
                pause_controls_handler.in_set(GameSystemSet::Encounter),
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

fn continue_button_handler(
    mut evw_transition: EventWriter<EncounterPauseStateTransitionEvent>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ContinueButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                evw_transition.send(EncounterPauseStateTransitionEvent);
            }
            Interaction::Hovered => {
                *color = CONTINUE_BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_NORMAL_COLOR.into();
            }
        }
    }
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

fn pause_controls_handler(
    mut evw_transition: EventWriter<EncounterPauseStateTransitionEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        evw_transition.send(EncounterPauseStateTransitionEvent);
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
    .with_children(game_logo)
    .with_children(continue_button)
    .with_children(main_menu_button);
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

fn continue_button(root: &mut ChildBuilder) {
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
        border_color: CONTINUE_BUTTON_HOVER_COLOR.into(),
        ..default()
    })
    .insert(ContinueButton)
    .with_children(continue_button_text);
}

fn continue_button_text(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section(CONTINUE_BUTTON_TEXT, TextStyle {
        font_size: CONTINUE_BUTTON_TEXT_SIZE,
        color: Color::WHITE,
        ..default()
    }));
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
