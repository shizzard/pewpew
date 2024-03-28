use std::borrow::Cow;

use bevy::prelude::*;

use crate::encounter::component::Health;

#[derive(Component, Debug, Default)]
pub struct Tag;

#[derive(Bundle, Default)]
pub struct HealthBundle {
    health: Health,
    bar: SpriteBundle,
    tag: Tag,
    name: Name,
}

impl HealthBundle {
    pub fn new(max_hp: f32, size: Vec2, name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            health: Health::new(max_hp),
            bar: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(0., 0., 1.)),
                ..default()
            },
            name: Name::new(name),
            ..default()
        }
    }
}
