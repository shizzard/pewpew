use bevy::prelude::*;

use super::component::MovableY;
use crate::GameSystemSet;

#[derive(Component, Debug)]
pub struct Tag;

#[derive(Bundle)]
pub struct Projectile {
    movable_y: MovableY,
    spatial: SpatialBundle,
}

impl Projectile {
    pub fn new(spawn_translation: Vec3, speed: f32) -> Self {
        Self {
            movable_y: MovableY {
                bound: (0., 1000.).into(),
                speed: speed.into(),
            },
            spatial: SpatialBundle::from_transform(Transform::from_translation(spawn_translation)),
        }
    }
}

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_projectile.in_set(GameSystemSet::EncounterPausable),
        );
    }
}

fn move_projectile(
    timer: Res<Time>,
    mut projectiles_query: Query<(&MovableY, &mut Transform), With<Tag>>,
) {
    projectiles_query
        .iter_mut()
        .for_each(|(projectile_movable, mut projectile_transform)| {
            projectile_movable.move_up(&mut projectile_transform, &timer);
        });
}
