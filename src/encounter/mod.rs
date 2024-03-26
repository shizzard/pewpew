pub mod arena;
pub mod component;

use bevy::prelude::*;

use self::arena::ArenaPlugin;
use crate::state::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncounterSetupSystemSet {
    PrepareArena,
    SpawnEntities,
}

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            OnEnter(GameState::Encounter),
            (
                EncounterSetupSystemSet::PrepareArena,
                EncounterSetupSystemSet::SpawnEntities.after(EncounterSetupSystemSet::PrepareArena),
            ),
        );
        app.add_plugins(ArenaPlugin);
    }
}
