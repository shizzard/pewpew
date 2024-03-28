use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::EncounterSetupSystemSet;
use crate::encounter::component::weapon::*;
use crate::encounter::component::*;
use crate::state::GameState;
use crate::GameSystemSet;

#[derive(Component, Debug, Default)]
pub struct Tag;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    movable: MovableX,
    size: EntitySize,
    spatial: SpatialBundle,
    weapon: Weapon,
    tag: Tag,
    name: Name,
}

const PLAYER_SPRITE_WIDTH: f32 = 40.;
const PLAYER_SPRITE_HEIGHT: f32 = 90.;

impl PlayerBundle {
    pub fn new(window: &Window, speed: f32) -> Self {
        let left_bound = (window.resolution.width() * super::ARENA_LAYOUT_SIDE_GAP_WIDTH_PERCENT)
            + PLAYER_SPRITE_WIDTH / 2.;
        let right_bound = (window.resolution.width()
            * (1. - super::ARENA_LAYOUT_SIDE_GAP_WIDTH_PERCENT))
            - PLAYER_SPRITE_WIDTH / 2.;
        let translation = (
            window.resolution.width() / 2.,
            (window.resolution.height() * super::ARENA_LAYOUT_GROUND_HEIGHT_PERCENT)
                + PLAYER_SPRITE_HEIGHT / 2.,
            1.,
        )
            .into();
        PlayerBundle {
            movable: MovableX {
                bound: (left_bound, right_bound).into(),
                speed: speed.into(),
            },
            size: (PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT).into(),
            spatial: SpatialBundle::from_transform(Transform::from_translation(translation)),
            weapon: Weapon::shotgun(),
            name: Name::new("Player"),
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
        )
        .add_systems(
            Update,
            movement_controls_handler.in_set(GameSystemSet::EncounterPausable),
        );
    }
}

const PLAYER_SPEED: f32 = 250.;
fn spawn_player_bundle(windows: Query<&Window, With<PrimaryWindow>>, mut cmd: Commands) {
    let window = windows.get_single().expect("Expected primary window");
    let player_bundle = PlayerBundle::new(window, PLAYER_SPEED);
    let sprite_bundle = player_bundle.sprite_bundle();
    cmd.spawn(player_bundle).with_children(|root| {
        root.spawn(sprite_bundle);
    });
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
