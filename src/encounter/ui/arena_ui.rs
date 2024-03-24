use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState;

const ROOT_NODE_COLOR: Color = Color::BLACK;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Component, Debug)]
pub struct ContinueButton;

#[derive(Component, Debug)]
pub struct MainMenuButton;

pub struct ArenaUIPlugin;

impl Plugin for ArenaUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Encounter), spawn_arena_ui)
            .add_systems(OnExit(GameState::Encounter), despawn_arena_ui);
    }
}

fn spawn_arena_ui(windows: Query<&Window, With<PrimaryWindow>>, mut cmd: Commands) {
    let window = windows.get_single().expect("Expected primary window");
    cmd.spawn(root_node(window))
        .with_children(ui_zone_1)
        .with_children(ui_zone_2)
        .with_children(ui_zone_3)
        .with_children(ui_zone_4)
        .insert(Tag);
}

fn despawn_arena_ui(mut cmd: Commands, query: Query<Entity, With<Tag>>) {
    let Ok(menu) = query.get_single() else {
        return;
    };
    cmd.entity(menu).despawn_recursive();
}

fn root_node(window: &Window) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(window.resolution.height() - window.resolution.width()),
            display: Display::Grid,
            padding: UiRect {
                left: Val::Px(25.0),
                right: Val::Px(25.0),
                top: Val::Px(25.0),
                ..default()
            },
            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
            grid_auto_flow: GridAutoFlow::Row,
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(10.0),
            ..default()
        },
        background_color: ROOT_NODE_COLOR.into(),
        ..default()
    }
}

const CONTROLS_TEXT_SIZE: f32 = 20.;
const CONTROLS_TEXT_COLOR: Color = Color::WHITE;
fn ui_zone_1(root: &mut ChildBuilder) {
    root.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "<A> <D>: MOVE\n".into(),
                    style: TextStyle {
                        font_size: CONTROLS_TEXT_SIZE,
                        color: CONTROLS_TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: "<SPACE>: FIRE\n".into(),
                    style: TextStyle {
                        font_size: CONTROLS_TEXT_SIZE,
                        color: CONTROLS_TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: "  <ESC>: MENU\n".into(),
                    style: TextStyle {
                        font_size: CONTROLS_TEXT_SIZE,
                        color: CONTROLS_TEXT_COLOR,
                        ..default()
                    },
                },
            ],
            ..default()
        },
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::MAROON.into(),
        ..default()
    });
}

fn ui_zone_2(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::DARK_GREEN.into(),
        ..default()
    });
}

fn ui_zone_3(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
    });
}

fn ui_zone_4(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::MIDNIGHT_BLUE.into(),
        ..default()
    });
}
