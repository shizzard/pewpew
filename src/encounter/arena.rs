use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

use crate::state::GameState;

#[derive(Component, Debug)]
pub struct Tag;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Encounter), spawn_encounter)
            .add_systems(OnExit(GameState::Encounter), despawn_encounter);
    }
}

fn spawn_encounter(windows: Query<&Window, With<PrimaryWindow>>, mut cmd: Commands) {
    let window = windows.get_single().expect("Expected primary window");
    cmd.spawn(Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "ENCOUNTER\n".to_string(),
                    style: TextStyle {
                        font_size: 50.,
                        color: Color::MAROON,
                        ..default()
                    },
                },
                TextSection {
                    value: "ARENA".to_string(),
                    style: TextStyle {
                        font_size: 50.,
                        color: Color::MAROON,
                        ..default()
                    },
                },
            ],
            justify: JustifyText::Center,
            ..default()
        },
        text_anchor: Anchor::Center,
        transform: Transform {
            translation: Vec3 {
                x: window.resolution.width() / 2.0,
                y: window.resolution.height() / 2.0,
                z: 0.0,
            },
            ..default()
        },
        ..default()
    })
    .insert(Tag);
}

fn despawn_encounter(mut cmd: Commands, query: Query<Entity, With<Tag>>) {
    let Ok(encounter) = query.get_single() else {
        return;
    };
    cmd.entity(encounter).despawn_recursive();
}
