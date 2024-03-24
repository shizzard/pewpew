use bevy::prelude::*;
use bevy::sprite::Anchor;

use self::swarm::EnemySwarm;
use self::swarm::MovementDirection;
use super::component::MovableX;
use super::component::MovableY;
use super::EncounterSetupSystemSet;
use crate::encounter::arena::BattleArenaTag;
use crate::state::GameState;
use crate::GameSystemSet;

pub mod swarm {
    use bevy::prelude::*;

    use crate::encounter::component::EntitySize;
    use crate::encounter::component::MovableX;
    use crate::encounter::component::MovableY;

    #[derive(Component, Debug)]
    pub struct Tag;

    #[derive(Component, Debug, Default)]
    pub enum MovementDirection {
        #[default]
        Left,
        Right,
    }

    #[derive(Bundle)]
    pub struct EnemySwarm {
        movable_x: MovableX,
        movable_y: MovableY,
        size: EntitySize,
        spatial: SpatialBundle,
    }

    pub const ENEMY_SWARM_WIDTH: f32 = 600.;
    pub const ENEMY_SWARM_HEIGHT: f32 = 300.;
    const ENEMY_SWARM_MARGIN: f32 = 50.;
    impl EnemySwarm {
        pub fn new(
            arena_width: f32,
            arena_height: f32,
            horizontal_speed: f32,
            vertical_speed: f32,
        ) -> Self {
            let left_bound = ENEMY_SWARM_MARGIN;
            let right_bound = arena_width - ENEMY_SWARM_WIDTH - ENEMY_SWARM_MARGIN;
            let top_bound: f32 = arena_height - ENEMY_SWARM_HEIGHT - ENEMY_SWARM_MARGIN;
            let bottom_bound: f32 = 0.;
            let translation = (
                (left_bound + (right_bound - left_bound) / 2.),
                top_bound,
                1.,
            )
                .into();
            EnemySwarm {
                movable_x: MovableX {
                    bound: (left_bound, right_bound).into(),
                    speed: horizontal_speed.into(),
                },
                movable_y: MovableY {
                    bound: (top_bound, bottom_bound).into(),
                    speed: vertical_speed.into(),
                },
                size: (ENEMY_SWARM_WIDTH, ENEMY_SWARM_HEIGHT).into(),
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
            move_enemy_swarm.in_set(GameSystemSet::EncounterPausable),
        );
    }
}

const ENEMY_SWARM_HORIZONTAL_SPEED: f32 = 50.;
const ENEMY_SWARM_VERTICAL_SPEED: f32 = 5.;
fn spawn_enemy_swarm_bundle(
    mut cmd: Commands,
    query: Query<(Entity, &Sprite), With<BattleArenaTag>>,
) {
    let (battle_arena_entity, battle_arena_sprite) =
        query.get_single().expect("Expected battle arena");
    let battle_arena_width = battle_arena_sprite.custom_size.unwrap();
    let enemy_swarm = cmd
        .spawn(EnemySwarm::new(
            battle_arena_width.x,
            battle_arena_width.y,
            ENEMY_SWARM_HORIZONTAL_SPEED,
            ENEMY_SWARM_VERTICAL_SPEED,
        ))
        .with_children(enemy_swarm_sprite)
        .insert(MovementDirection::default())
        .insert(Name::new("Enemy Swarm"))
        .insert(swarm::Tag)
        .id();
    cmd.entity(battle_arena_entity).add_child(enemy_swarm);
}

fn enemy_swarm_sprite(root: &mut ChildBuilder) {
    root.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2 {
                x: swarm::ENEMY_SWARM_WIDTH,
                y: swarm::ENEMY_SWARM_HEIGHT,
            }),
            anchor: Anchor::BottomLeft,
            ..Sprite::default()
        },
        ..SpriteBundle::default()
    });
}

fn move_enemy_swarm(
    timer: Res<Time>,
    mut query: Query<
        (&mut MovementDirection, &MovableX, &MovableY, &mut Transform),
        With<swarm::Tag>,
    >,
) {
    let (
        mut last_movement_direction,
        enemy_swarm_movable_x,
        enemy_swarm_movable_y,
        mut enemy_swarm_transform,
    ) = query.get_single_mut().expect("Expected enemy swarm");
    match last_movement_direction.as_ref() {
        MovementDirection::Left => {
            if enemy_swarm_movable_x.can_move_left(&mut enemy_swarm_transform) {
                enemy_swarm_movable_x.move_left(&mut enemy_swarm_transform, &timer);
            } else {
                *last_movement_direction = MovementDirection::Right;
            }
        }
        MovementDirection::Right => {
            if enemy_swarm_movable_x.can_move_right(&mut enemy_swarm_transform) {
                enemy_swarm_movable_x.move_right(&mut enemy_swarm_transform, &timer);
            } else {
                *last_movement_direction = MovementDirection::Left;
            }
        }
    }
    enemy_swarm_movable_y.move_down(&mut enemy_swarm_transform, &timer);
}
