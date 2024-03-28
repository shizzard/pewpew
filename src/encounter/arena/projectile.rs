use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;

use crate::encounter::component::weapon::*;
use crate::encounter::component::*;
use crate::GameSystemSet;

#[derive(Component, Debug, Default)]
pub struct Tag;

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    velocity: Velocity,
    spatial: SpatialBundle,
    tag: Tag,
    name: Name,
}

impl ProjectileBundle {
    pub fn new(velocity: Velocity, transform: Transform) -> Self {
        Self {
            velocity,
            spatial: SpatialBundle::from_transform(transform),
            name: Name::new("Shot"),
            ..default()
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
            (hit_controls_handler, despawn_enemies, redraw_health_sprites)
                .chain()
                .in_set(GameSystemSet::EncounterPausable),
        );
    }
}

fn move_projectile(
    timer: Res<Time>,
    mut cmd: Commands,
    mut projectiles_query: Query<(Entity, &Velocity, &mut Transform), With<Tag>>,
) {
    for (projectile_entity, projectile_velocity, mut projectile_transform) in &mut projectiles_query
    {
        if projectile_transform.translation.x > 2000. {
            cmd.entity(projectile_entity).despawn_recursive();
            return;
        }
        projectile_velocity.advance(&timer, &mut projectile_transform);
    }
}

fn fire_controls_handler(
    mut cmd: Commands,
    rng: ResMut<GlobalEntropy<WyRand>>,
    global_timer: Res<Time>,
    mut player_query: Query<(&Transform, &mut Weapon), With<crate::encounter::arena::player::Tag>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (player_transform, mut player_weapon) =
        player_query.get_single_mut().expect("Expected player");
    player_weapon.tick(global_timer);
    if keyboard_input.pressed(KeyCode::Space) {
        match player_weapon.shoot(player_transform.translation, rng) {
            Some((shots, shot_sprite)) => {
                for (velocity, transform) in shots.into_iter() {
                    let projectile = ProjectileBundle::new(velocity, transform);
                    cmd.spawn(projectile).with_children(|root| {
                        root.spawn(shot_sprite.clone());
                    });
                }
            }
            None => (),
        }
    }
}

fn hit_controls_handler(
    mut cmd: Commands,
    weapon_query: Query<&Weapon, With<crate::encounter::arena::player::Tag>>,
    mut projectiles_query: Query<(Entity, &Transform), With<super::projectile::Tag>>,
    enemies_query: Query<&Transform, With<super::enemy::Tag>>,
    mut health_query: Query<(&mut Health, &Parent), With<super::health::Tag>>,
) {
    let player_weapon = weapon_query.get_single().expect("Expected player");

    let mut enemies = vec![];
    for (health, parent) in health_query.iter_mut() {
        let enemy_transform = enemies_query
            .get(parent.get())
            .expect("Health component without parent enemy");
        enemies.push((health, enemy_transform));
    }

    for (projectile_entity, projectile_transform) in &mut projectiles_query {
        match player_weapon.try_apply_damage(projectile_transform, &mut enemies) {
            Some(applied_damage) => {
                bevy::log::info!("Applied damage: {}", applied_damage);
                cmd.entity(projectile_entity).despawn_recursive()
            }
            None => (),
        }
    }
}

fn despawn_enemies(
    mut cmd: Commands,
    enemies_query: Query<Entity, With<super::enemy::Tag>>,
    mut health_query: Query<(&mut Health, &Parent), With<super::health::Tag>>,
) {
    for (health, parent) in health_query.iter_mut() {
        let enemy_entity = enemies_query
            .get(parent.get())
            .expect("Health component without parent enemy");
        if health.dead() {
            cmd.entity(enemy_entity).despawn_recursive();
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
