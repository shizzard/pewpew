use bevy::prelude::*;

use super::component::MovableX;
use super::transition::EncounterPauseStateTransitionEvent;
use crate::GameSystemSet;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (pause_controls_handler, movement_controls_handler).in_set(GameSystemSet::Encounter),
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
