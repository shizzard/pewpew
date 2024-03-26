use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::encounter::component::*;
use crate::GameSystemSet;

#[derive(Component, Debug, Default)]
pub struct Tag;

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    movable_y: MovableY,
    spatial: SpatialBundle,
    tag: Tag,
}

impl ProjectileBundle {
    pub fn new(spawn_translation: Vec3, speed: f32) -> Self {
        Self {
            movable_y: MovableY {
                bound: (0., 1000.).into(),
                speed: speed.into(),
            },
            spatial: SpatialBundle::from_transform(Transform::from_translation(spawn_translation)),
            ..default()
        }
    }

    pub fn sprite(&self) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2 { x: 2., y: 8. }),
                anchor: Anchor::Center,
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        }
    }
}

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_projectile.in_set(GameSystemSet::EncounterPausable),
        )
        .add_systems(
            Update,
            fire_controls_handler.in_set(GameSystemSet::EncounterPausable),
        )
        .add_systems(
            Update,
            (hit_controls_handler, redraw_health_sprites)
                .chain()
                .in_set(GameSystemSet::EncounterPausable),
        );
    }
}

fn move_projectile(
    timer: Res<Time>,
    mut cmd: Commands,
    mut projectiles_query: Query<(Entity, &MovableY, &mut Transform), With<Tag>>,
) {
    for (projectile_entity, projectile_movable, mut projectile_transform) in &mut projectiles_query
    {
        match projectile_movable.can_move_up(&mut projectile_transform) {
            false => cmd.entity(projectile_entity).despawn_recursive(),
            true => projectile_movable.move_up(&mut projectile_transform, &timer),
        };
    }
}

fn fire_controls_handler(
    mut cmd: Commands,
    player_transform_query: Query<&Transform, With<crate::encounter::arena::player::Tag>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let player_transform = player_transform_query
        .get_single()
        .expect("Expected player");
    let projectile = ProjectileBundle::new(player_transform.translation, 500.);
    let projectile_sprite = projectile.sprite();
    if keyboard_input.just_pressed(KeyCode::Space) {
        cmd.spawn(projectile).with_children(|root| {
            root.spawn(projectile_sprite);
        });
    }
}

const HIT_RANGE: f32 = 15.;
const HIT_DAMAGE: f32 = 15.;
fn hit_controls_handler(
    mut cmd: Commands,
    mut projectiles_query: Query<(Entity, &Transform), With<super::projectile::Tag>>,
    mut enemies_query: Query<(Entity, &Name, &Children, &Transform), With<super::enemy::Tag>>,
    mut health_query: Query<&mut Health, With<super::health::Tag>>,
) {
    for (projectile_entity, projectile_transform) in &mut projectiles_query {
        for (enemy_entity, _enemy_name, enemy_children, enemy_transform) in &mut enemies_query {
            if projectile_transform
                .translation
                .distance(enemy_transform.translation)
                <= HIT_RANGE
            {
                for &child in enemy_children {
                    let Ok(mut enemy_health) = health_query.get_mut(child) else {
                        continue;
                    };
                    enemy_health.actual -= HIT_DAMAGE;
                    cmd.entity(projectile_entity).despawn_recursive();
                    if enemy_health.dead() {
                        cmd.entity(enemy_entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn redraw_health_sprites(
    mut enemies_query: Query<(&Name, &Children), With<super::enemy::Tag>>,
    mut health_query: Query<(&Health, &mut Transform), With<super::health::Tag>>,
) {
    for (_enemy_name, enemy_children) in &mut enemies_query {
        for &child in enemy_children {
            let Ok(enemy_health_components) = health_query.get_mut(child) else {
                continue;
            };
            let (enemy_health, mut enemy_health_sprite_transform) = enemy_health_components;
            enemy_health_sprite_transform.scale = Vec3::splat(1. - enemy_health.ratio());
        }
    }
}
