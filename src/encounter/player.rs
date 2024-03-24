use bevy::prelude::*;

use super::component::EntitySize;
use super::component::MovableX;
use super::EncounterSetupSystemSet;
use crate::encounter::arena::BattleArenaTag;
use crate::state::GameState;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    movable: MovableX,
    size: EntitySize,
    spatial: SpatialBundle,
}

const PLAYER_SPRITE_WIDTH: f32 = 50.;
const PLAYER_SPRITE_HEIGHT: f32 = 50.;

impl PlayerBundle {
    pub fn new(arena_width: f32, speed: f32) -> Self {
        let left_bound = PLAYER_SPRITE_WIDTH / 2.;
        let right_bound = arena_width - PLAYER_SPRITE_WIDTH / 2.;
        let translation = (arena_width / 2., PLAYER_SPRITE_HEIGHT / 2., 1.).into();
        PlayerBundle {
            movable: MovableX {
                bound: (left_bound, right_bound).into(),
                speed: speed.into(),
            },
            size: (PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT).into(),
            spatial: SpatialBundle {
                transform: Transform {
                    translation,
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }

    pub fn sprite_bundle(&self) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: Color::MAROON,
                custom_size: Some(self.size.vec),
                anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
            ..default()
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
    let player_bundle = PlayerBundle::new(battle_arena_width.x, PLAYER_SPEED);
    let sprite_bundle = player_bundle.sprite_bundle();
    let player = cmd
        .spawn(player_bundle)
        .with_children(|root| {
            root.spawn(sprite_bundle);
        })
        .insert(Name::new("Player"))
        .insert(Tag)
        .id();
    cmd.entity(battle_arena_entity).add_child(player);
}
