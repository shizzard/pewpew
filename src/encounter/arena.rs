use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

use super::enemy::EnemyPlugin;
use super::player::PlayerPlugin;
use super::ui::arena_ui::ArenaUIPlugin;
use super::EncounterSetupSystemSet;
use crate::state::GameState;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Component, Debug)]
pub struct BattleArenaTag;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(ArenaUIPlugin)
            .add_systems(
                OnEnter(GameState::Encounter),
                spawn_arena.in_set(EncounterSetupSystemSet::PrepareArena),
            )
            .add_systems(OnExit(GameState::Encounter), despawn_arena);
    }
}

const BATTLE_ARENA_MARGIN_PX: f32 = 25.;
fn spawn_arena(windows: Query<&Window, With<PrimaryWindow>>, mut cmd: Commands) {
    let window = windows.get_single().expect("Expected primary window");
    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Vec2 {
                x: window.resolution.width(),
                y: window.resolution.width(),
            }
            .into(),
            color: Color::BLACK,
            anchor: Anchor::BottomLeft,
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        root.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2 {
                    x: window.resolution.width() - BATTLE_ARENA_MARGIN_PX * 2.,
                    y: window.resolution.width() - BATTLE_ARENA_MARGIN_PX * 2.,
                }
                .into(),
                color: Color::NAVY,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: BATTLE_ARENA_MARGIN_PX,
                    y: BATTLE_ARENA_MARGIN_PX,
                    z: 1.,
                },
                ..default()
            },
            ..default()
        })
        .insert(Name::new("BattleArena"))
        .insert(BattleArenaTag);
    })
    .insert(Name::new("Arena"))
    .insert(Tag);
}

fn despawn_arena(mut cmd: Commands, query: Query<Entity, With<Tag>>) {
    let Ok(encounter) = query.get_single() else {
        return;
    };
    cmd.entity(encounter).despawn_recursive();
}
