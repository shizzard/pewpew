pub mod arena;
pub mod component;
pub mod controls;
pub mod enemy;
pub mod player;
pub mod projectile;
pub mod ui;

use bevy::prelude::*;

use self::arena::ArenaPlugin;
use self::controls::ControlsPlugin;
use self::transition::PauseStateTransitionPlugin;
use self::ui::pause_menu::PauseMenuUIPlugin;
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
        app.add_plugins(PauseStateTransitionPlugin)
            .add_plugins(ControlsPlugin)
            .add_plugins(ArenaPlugin)
            .add_plugins(PauseMenuUIPlugin);
    }
}

pub mod transition {
    use bevy::ecs::schedule::States;
    use bevy::prelude::*;

    use crate::state::GameState;
    use crate::GameSystemSet;

    #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum PauseState {
        #[default]
        Running,
        Pause,
    }

    pub struct PauseStateTransitionPlugin;

    impl Plugin for PauseStateTransitionPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<PauseState>()
                .add_systems(OnExit(GameState::Encounter), on_exit_encounter)
                .add_systems(
                    Update,
                    handle_encounter_pause_transition.in_set(GameSystemSet::Encounter),
                )
                .add_event::<EncounterPauseStateTransitionEvent>();
        }
    }

    /// This system ensures that player will not leave the encounter with pause
    /// turned on.
    ///
    /// The reason for this workaround is that GameState and PauseState are
    /// completely independent for now, so there are no appropriate ways to
    /// switch the PauseState based on GameState, except this one.
    ///
    /// If you turn this system off, then, if player exits the encounter, pause
    /// menu will hang on the screen forever.
    ///
    /// TODO: This may be fixed when sub-states support will land:
    /// https://github.com/bevyengine/bevy/pull/11426
    fn on_exit_encounter(mut next_state: ResMut<NextState<PauseState>>) {
        next_state.set(PauseState::Running);
    }

    #[derive(Event, Debug)]
    pub struct EncounterPauseStateTransitionEvent;

    fn handle_encounter_pause_transition(
        state: Res<State<PauseState>>,
        mut evr_transition: EventReader<EncounterPauseStateTransitionEvent>,
        mut next_state: ResMut<NextState<PauseState>>,
    ) {
        // this transition is effective only once, despite we can have a few events in the queue
        for _ev in evr_transition.read() {
            match state.get() {
                PauseState::Running => {
                    bevy::log::info!("Setting game on pause");
                    next_state.set(PauseState::Pause);
                }
                PauseState::Pause => {
                    bevy::log::info!("Setting game running");
                    next_state.set(PauseState::Running)
                }
            }
        }
    }
}
