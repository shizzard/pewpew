use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand::Rng;

use super::health::HealthBundle;
use super::EncounterSetupSystemSet;
use crate::encounter::component::*;
use crate::state::GameState;
use crate::transition::GameStateTransitionEvent;
use crate::GameSystemSet;

#[derive(Component, Debug, Default)]
pub struct Tag;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    size: EntitySize,
    movable_x: MovableX,
    movable_y: MovableY,
    movement_direction: MovementDirectionX,
    spatial: SpatialBundle,
    tag: Tag,
    name: Name,
}

pub const ENEMY_ENTITY_WIDTH: f32 = 30.;
pub const ENEMY_ENTITY_HEIGHT: f32 = 30.;
impl EnemyBundle {
    pub fn new(n: usize, window: &Window, rng: &mut ResMut<GlobalEntropy<WyRand>>) -> Self {
        let left_bound = super::battle_arena_left_bound(window) + ENEMY_ENTITY_WIDTH / 2.;
        let right_bound = super::battle_arena_right_bound(window) - ENEMY_ENTITY_WIDTH / 2.;
        let top_bound: f32 = super::battle_arena_top_bound(window) - ENEMY_ENTITY_HEIGHT / 2.;
        let bottom_bound: f32 = super::battle_arena_bottom_bound(window) + ENEMY_ENTITY_HEIGHT / 2.;
        let spawn_area_top = super::battle_arena_spawn_top(window);
        let spawn_area_bottom = super::battle_arena_spawn_bottom(window);
        let spawn_vec = Vec2::new(
            rng.gen_range(left_bound..right_bound),
            rng.gen_range(spawn_area_bottom..spawn_area_top),
        );
        let horizontal_speed =
            rng.gen_range(ENEMY_HORIZONTAL_SPEED_MIN..ENEMY_HORIZONTAL_SPEED_MAX);
        let vertical_speed = rng.gen_range(ENEMY_VERTICAL_SPEED_MIN..ENEMY_VERTICAL_SPEED_MAX);
        let translation = (spawn_vec.x, spawn_vec.y, 1.).into();

        Self {
            size: (ENEMY_ENTITY_WIDTH, ENEMY_ENTITY_HEIGHT).into(),
            movable_x: MovableX {
                bound: (left_bound, right_bound).into(),
                speed: horizontal_speed.into(),
            },
            movable_y: MovableY {
                bound: (bottom_bound, top_bound).into(),
                speed: vertical_speed.into(),
            },
            movement_direction: MovementDirectionX::default(),
            spatial: SpatialBundle::from_transform(Transform::from_translation(translation)),
            name: Name::new(format!("Enemy #{}", n)),
            ..default()
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

    pub fn health_bundle(&self, n: usize) -> HealthBundle {
        HealthBundle::new(
            100.,
            Vec2::new(ENEMY_ENTITY_WIDTH, ENEMY_ENTITY_HEIGHT),
            format!("Enemy #{} Health", n),
        )
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
        )
        .add_systems(
            Update,
            game_over_swarm_criteria_handler.in_set(GameSystemSet::EncounterPausable),
        );
    }
}

const ENEMY_HORIZONTAL_SPEED_MIN: f32 = 30.;
const ENEMY_HORIZONTAL_SPEED_MAX: f32 = 100.;
const ENEMY_VERTICAL_SPEED_MIN: f32 = 5.;
const ENEMY_VERTICAL_SPEED_MAX: f32 = 15.;
const ENEMY_SWARM_COUNT: usize = 50;
fn spawn_enemy_swarm_bundle(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cmd: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let window = windows.get_single().expect("Expected primary window");
    for n in 0..ENEMY_SWARM_COUNT {
        let enemy = EnemyBundle::new(n, window, &mut rng);
        let enemy_sprite = enemy.sprite();
        let enemy_health = enemy.health_bundle(n);
        cmd.spawn(enemy)
            .with_children(|root| {
                root.spawn(enemy_sprite);
            })
            .with_children(|root| {
                root.spawn(enemy_health);
            });
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

fn game_over_swarm_criteria_handler(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut evw_transition: EventWriter<GameStateTransitionEvent>,
    enemy_entities_query: Query<(&Transform, &Name), With<super::enemy::Tag>>,
) {
    let window = windows.get_single().expect("Expected primary window");
    let touchdown_y = super::battle_arena_bottom_bound(window) + ENEMY_ENTITY_HEIGHT / 2.;
    if enemy_entities_query
        .iter()
        .any(|(entity_transform, _name)| entity_transform.translation.y <= touchdown_y)
    {
        evw_transition.send(GameStateTransitionEvent::GameOver);
    }
}
