use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::component::MovableX;
use super::EncounterSetupSystemSet;
use crate::encounter::arena::BattleArenaTag;
use crate::state::GameState;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Bundle)]
pub struct PlayerBundle {
    movable: MovableX,
    spatial: SpatialBundle,
}

impl PlayerBundle {
    pub fn new(
        translation: Vec3,
        movement_left_bound: f32,
        movement_right_bound: f32,
        speed: f32,
    ) -> Self {
        PlayerBundle {
            movable: MovableX {
                bound: (movement_left_bound, movement_right_bound).into(),
                speed: speed.into(),
            },
            spatial: SpatialBundle {
                transform: Transform {
                    translation,
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Encounter),
            spawn_player_bundle.in_set(EncounterSetupSystemSet::SpawnEntities),
        );
    }
}

const PLAYER_SPEED: f32 = 250.;
fn spawn_player_bundle(mut cmd: Commands, query: Query<(Entity, &Sprite), With<BattleArenaTag>>) {
    let (battle_arena_entity, battle_arena_sprite) =
        query.get_single().expect("Expected battle arena");
    let battle_arena_width = battle_arena_sprite.custom_size.unwrap();
    let player = cmd
        .spawn(PlayerBundle::new(
            (battle_arena_width.x / 2., 0., 0.).into(),
            PLAYER_SPRITE_WIDTH / 2.,
            battle_arena_width.x - PLAYER_SPRITE_WIDTH / 2.,
            PLAYER_SPEED,
        ))
        .with_children(player_sprite)
        .insert(Tag)
        .id();
    cmd.entity(battle_arena_entity).add_child(player);
}

const PLAYER_SPRITE_WIDTH: f32 = 50.;
const PLAYER_SPRITE_HEIGHT: f32 = 50.;
fn player_sprite(root: &mut ChildBuilder) {
    root.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::MAROON,
            custom_size: Some(Vec2 {
                x: PLAYER_SPRITE_WIDTH,
                y: PLAYER_SPRITE_HEIGHT,
            }),
            anchor: Anchor::BottomCenter,
            ..Sprite::default()
        },
        ..SpriteBundle::default()
    });
}
