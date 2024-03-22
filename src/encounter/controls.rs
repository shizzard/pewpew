use bevy::prelude::*;

use super::transition::EncounterPauseStateTransitionEvent;
use crate::GameSystemSet;

#[derive(Component, Debug)]
pub struct Tag;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            pause_controls_handler.in_set(GameSystemSet::Encounter),
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
