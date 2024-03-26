pub mod enemy;
pub mod health;
pub mod player;
pub mod projectile;
pub mod ui;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use ui::ArenaUIPlugin;

use self::projectile::ProjectilePlugin;
use super::EncounterSetupSystemSet;
use crate::state::GameState;
use crate::util::ColorHex;

pub const ARENA_LAYOUT_GROUND_HEIGHT_PERCENT: f32 = 0.05;
pub const ARENA_LAYOUT_BATTLE_ARENA_HEIGHT_PERCENT: f32 = 0.85;
pub const ARENA_LAYOUT_UI_HEIGHT_PERCENT: f32 = 0.1;
pub const ARENA_LAYOUT_BATTLE_ARENA_ENEMY_SPAWN_HEIGHT_PERCENT: f32 = 0.2;
pub const ARENA_LAYOUT_SIDE_GAP_WIDTH_PERCENT: f32 = 0.05;

#[derive(Component, Debug, Default)]
pub struct Tag;

#[derive(Bundle, Default)]
pub struct ArenaGroundBundle {
    sprite: SpriteBundle,
    tag: Tag,
}
impl ArenaGroundBundle {
    pub fn new(window: &Window) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2 {
                        x: window.resolution.width(),
                        y: window.resolution.height() * ARENA_LAYOUT_GROUND_HEIGHT_PERCENT,
                    }
                    .into(),
                    color: ColorHex::new(0x2e4d2c).into(),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}

pub fn battle_arena_left_bound(window: &Window) -> f32 {
    window.resolution.width() * ARENA_LAYOUT_SIDE_GAP_WIDTH_PERCENT
}

pub fn battle_arena_right_bound(window: &Window) -> f32 {
    window.resolution.width() * (1. - ARENA_LAYOUT_SIDE_GAP_WIDTH_PERCENT)
}

pub fn battle_arena_top_bound(window: &Window) -> f32 {
    window.resolution.height()
        * (ARENA_LAYOUT_GROUND_HEIGHT_PERCENT + ARENA_LAYOUT_BATTLE_ARENA_HEIGHT_PERCENT)
}

pub fn battle_arena_bottom_bound(window: &Window) -> f32 {
    window.resolution.height() * ARENA_LAYOUT_GROUND_HEIGHT_PERCENT
}

pub fn battle_arena_spawn_top(window: &Window) -> f32 {
    battle_arena_top_bound(window)
}

pub fn battle_arena_spawn_bottom(window: &Window) -> f32 {
    window.resolution.height()
        * (ARENA_LAYOUT_GROUND_HEIGHT_PERCENT + ARENA_LAYOUT_BATTLE_ARENA_HEIGHT_PERCENT
            - ARENA_LAYOUT_BATTLE_ARENA_ENEMY_SPAWN_HEIGHT_PERCENT)
}

pub fn ui_height(window: &Window) -> f32 {
    window.resolution.height() * ARENA_LAYOUT_UI_HEIGHT_PERCENT
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(ProjectilePlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(ArenaUIPlugin)
            .add_systems(
                OnEnter(GameState::Encounter),
                spawn_arena.in_set(EncounterSetupSystemSet::PrepareArena),
            )
            .add_systems(OnExit(GameState::Encounter), despawn_arena);
    }
}

fn spawn_arena(windows: Query<&Window, With<PrimaryWindow>>, mut cmd: Commands) {
    let window = windows.get_single().expect("Expected primary window");
    cmd.spawn(ArenaGroundBundle::new(window));
}

fn despawn_arena(mut cmd: Commands, query: Query<Entity, With<Tag>>) {
    let Ok(encounter) = query.get_single() else {
        return;
    };
    cmd.entity(encounter).despawn_recursive();
}
