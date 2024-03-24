use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::arena::BattleArenaTag;
use super::component::MovableX;
use super::projectile::Projectile;
use super::transition::EncounterPauseStateTransitionEvent;
use crate::transition::GameStateTransitionEvent;
use crate::GameSystemSet;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            pause_controls_handler.in_set(GameSystemSet::Encounter),
        )
        .add_systems(
            Update,
            movement_controls_handler.in_set(GameSystemSet::EncounterPausable),
        )
        .add_systems(
            Update,
            fire_controls_handler.in_set(GameSystemSet::EncounterPausable),
        )
        .add_systems(
            Update,
            hit_controls_handler.in_set(GameSystemSet::EncounterPausable),
        )
        .add_systems(
            Update,
            game_over_swarm_criteria_handler.in_set(GameSystemSet::EncounterPausable),
        );
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

fn movement_controls_handler(
    timer: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&MovableX, &mut Transform), With<super::player::Tag>>,
) {
    let (player_movable, mut player_transform) = query.get_single_mut().expect("Expected player");
    if keyboard_input.pressed(KeyCode::KeyA) {
        player_movable.move_left(player_transform.as_mut(), &timer);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_movable.move_right(player_transform.as_mut(), &timer);
    }
}

fn fire_controls_handler(
    mut cmd: Commands,
    battle_arena_query: Query<Entity, With<BattleArenaTag>>,
    player_transform_query: Query<&Transform, With<crate::encounter::player::Tag>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let battle_arena_entity = battle_arena_query
        .get_single()
        .expect("Expected battle arena");
    let player_transform = player_transform_query
        .get_single()
        .expect("Expected player");
    if keyboard_input.just_pressed(KeyCode::Space) {
        let projectile = cmd
            .spawn(Projectile::new(player_transform.translation, 500.))
            .with_children(projectile_sprite)
            .insert(super::projectile::Tag)
            .id();
        cmd.entity(battle_arena_entity).add_child(projectile);
    }
}

const HIT_RANGE: f32 = 15.;
fn hit_controls_handler(
    mut cmd: Commands,
    battle_arena_query: Query<Entity, With<BattleArenaTag>>,
    mut projectiles_query: Query<(Entity, &Transform), With<super::projectile::Tag>>,
    mut enemies_query: Query<(Entity, &Transform), With<super::enemy::Tag>>,
) {
    let battle_arena_entity = battle_arena_query
        .get_single()
        .expect("Expected battle arena");
    projectiles_query
        .iter_mut()
        .for_each(|(projectile_entity, projectile_transform)| {
            enemies_query
                .iter_mut()
                .for_each(|(enemy_entity, enemy_transform)| {
                    if projectile_transform
                        .translation
                        .distance(enemy_transform.translation)
                        <= HIT_RANGE
                    {
                        cmd.entity(projectile_entity).despawn_recursive();
                        cmd.entity(enemy_entity).despawn_recursive();
                        cmd.entity(battle_arena_entity)
                            .remove_children(&[projectile_entity, enemy_entity]);
                    }
                })
        });
}

pub fn projectile_sprite(root: &mut ChildBuilder) {
    root.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2 { x: 5., y: 5. }),
            anchor: Anchor::Center,
            ..Sprite::default()
        },
        ..SpriteBundle::default()
    });
}

fn game_over_swarm_criteria_handler(
    mut evw_transition: EventWriter<GameStateTransitionEvent>,
    battle_arena_query: Query<&GlobalTransform, With<BattleArenaTag>>,
    enemy_entities_query: Query<(&GlobalTransform, &Name), With<super::enemy::Tag>>,
) {
    let battle_arena_transform = battle_arena_query
        .get_single()
        .expect("Expected battle arena");
    bevy::log::info!("{:?}", enemy_entities_query);
    if enemy_entities_query
        .iter()
        .any(|(entity_transform, _name)| {
            battle_arena_transform.translation().y > entity_transform.translation().y
        })
    {
        evw_transition.send(GameStateTransitionEvent::GameOver);
    }
}
