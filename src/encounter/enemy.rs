use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand::Rng;

use super::component::EntitySize;
use super::component::MovableX;
use super::component::MovableY;
use super::component::MovementDirectionX;
use super::EncounterSetupSystemSet;
use crate::encounter::arena::BattleArenaTag;
use crate::state::GameState;
use crate::GameSystemSet;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Bundle)]
pub struct Enemy {
    size: EntitySize,
    movable_x: MovableX,
    movable_y: MovableY,
    spatial: SpatialBundle,
}

pub const ENEMY_ENTITY_WIDTH: f32 = 30.;
pub const ENEMY_ENTITY_HEIGHT: f32 = 30.;
impl Enemy {
    pub fn new(
        x: f32,
        y: f32,
        arena_size: Vec2,
        horizontal_speed: f32,
        vertical_speed: f32,
    ) -> Self {
        let translation = (x, y, 1.).into();
        Enemy {
            size: (ENEMY_ENTITY_WIDTH, ENEMY_ENTITY_HEIGHT).into(),
            movable_x: MovableX {
                bound: (
                    ENEMY_ENTITY_WIDTH / 2.,
                    arena_size.x - ENEMY_ENTITY_WIDTH / 2.,
                )
                    .into(),
                speed: horizontal_speed.into(),
            },
            movable_y: MovableY {
                bound: (
                    ENEMY_ENTITY_HEIGHT / 2.,
                    arena_size.y - ENEMY_ENTITY_HEIGHT / 2.,
                )
                    .into(),
                speed: vertical_speed.into(),
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

    pub fn sprite(&self) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2 {
                    x: ENEMY_ENTITY_WIDTH,
                    y: ENEMY_ENTITY_HEIGHT,
                }),
                anchor: Anchor::Center,
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        }
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Encounter),
            spawn_enemy_swarm_bundle.in_set(EncounterSetupSystemSet::SpawnEntities),
        )
        .add_systems(
            Update,
            move_enemies.in_set(GameSystemSet::EncounterPausable),
        );
    }
}

const ENEMY_HORIZONTAL_SPEED_MIN: f32 = 30.;
const ENEMY_HORIZONTAL_SPEED_MAX: f32 = 100.;
const ENEMY_VERTICAL_SPEED_MIN: f32 = 5.;
const ENEMY_VERTICAL_SPEED_MAX: f32 = 15.;
const ENEMY_SWARM_COUNT: u8 = 50;
fn spawn_enemy_swarm_bundle(
    mut cmd: Commands,
    query: Query<(Entity, &Sprite), With<BattleArenaTag>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let (battle_arena_entity, battle_arena_sprite) =
        query.get_single().expect("Expected battle arena");
    let battle_arena_size = battle_arena_sprite.custom_size.unwrap();
    for n in 0..ENEMY_SWARM_COUNT {
        let enemy = Enemy::new(
            rng.gen_range((battle_arena_size.x * 0.1)..(battle_arena_size.x * 0.9)),
            rng.gen_range((battle_arena_size.y * 0.6)..(battle_arena_size.y * 0.9)),
            battle_arena_size,
            rng.gen_range(ENEMY_HORIZONTAL_SPEED_MIN..ENEMY_HORIZONTAL_SPEED_MAX),
            rng.gen_range(ENEMY_VERTICAL_SPEED_MIN..ENEMY_VERTICAL_SPEED_MAX),
        );
        let enemy_sprite = enemy.sprite();
        let enemy = cmd
            .spawn(enemy)
            .with_children(|root| {
                root.spawn(enemy_sprite);
            })
            .insert(Name::new(format!("Enemy Entity #{}", n)))
            .insert(match rng.gen_bool(0.5) {
                true => MovementDirectionX::Left,
                false => MovementDirectionX::Right,
            })
            .insert(Tag)
            .id();
        cmd.entity(battle_arena_entity).add_child(enemy);
    }
}

fn move_enemies(
    timer: Res<Time>,
    mut query: Query<
        (
            &mut MovementDirectionX,
            &MovableX,
            &MovableY,
            &mut Transform,
        ),
        With<Tag>,
    >,
) {
    query.iter_mut().for_each(
        |(mut last_movement_direction, enemy_movable_x, enemy_movable_y, mut enemy_transform)| {
            match last_movement_direction.as_ref() {
                MovementDirectionX::Left => {
                    if enemy_movable_x.can_move_left(&mut enemy_transform) {
                        enemy_movable_x.move_left(&mut enemy_transform, &timer);
                    } else {
                        *last_movement_direction = MovementDirectionX::Right;
                    }
                }
                MovementDirectionX::Right => {
                    if enemy_movable_x.can_move_right(&mut enemy_transform) {
                        enemy_movable_x.move_right(&mut enemy_transform, &timer);
                    } else {
                        *last_movement_direction = MovementDirectionX::Left;
                    }
                }
            }
            enemy_movable_y.move_down(&mut enemy_transform, &timer);
        },
    );
}
