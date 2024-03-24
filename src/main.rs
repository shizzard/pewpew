pub mod encounter;
pub mod global;
pub mod main_menu;

use bevy::prelude::*;
use encounter::transition::PauseState;
use state::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSystemSet {
    Global,
    MainMenu,
    Encounter,
    EncounterPausable,
    GameOver,
}

fn main() {
    let mut app = App::new();
    app.configure_sets(
        Update,
        (
            GameSystemSet::Global,
            GameSystemSet::MainMenu
                .run_if(in_state(GameState::MainMenu))
                .after(GameSystemSet::Global),
            GameSystemSet::Encounter
                .run_if(in_state(GameState::Encounter))
                .after(GameSystemSet::Global),
            GameSystemSet::EncounterPausable
                .run_if(in_state(GameState::Encounter))
                .run_if(in_state(PauseState::Running))
                .after(GameSystemSet::Global),
        ),
    );

    app.add_plugins(global::GlobalPlugin)
        .add_plugins(transition::GameStateTransitionPlugin)
        .add_plugins(main_menu::ui::MainMenuUIPlugin)
        .add_plugins(encounter::EncounterPlugin);

    app.run();
}

pub mod transition {
    use bevy::app::AppExit;
    use bevy::prelude::*;

    use super::state::GameState;
    use super::GameSystemSet;

    #[derive(Event, Debug)]
    pub enum GameStateTransitionEvent {
        StartEncounter,
        QuitEncounter,
        GameOver,
        QuitGame,
    }

    pub struct GameStateTransitionPlugin;

    impl Plugin for GameStateTransitionPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<GameState>()
                .add_systems(
                    Update,
                    handle_game_state_transition.in_set(GameSystemSet::Global),
                )
                .add_event::<GameStateTransitionEvent>();
        }
    }

    fn handle_game_state_transition(
        state: Res<State<GameState>>,
        mut evr_transition: EventReader<GameStateTransitionEvent>,
        mut next_state: ResMut<NextState<GameState>>,
        mut evw_exit: EventWriter<AppExit>,
    ) {
        let state = state.get();
        let Some(ev) = evr_transition.read().next() else {
            return;
        };
        match (state, ev) {
            (GameState::MainMenu, GameStateTransitionEvent::StartEncounter) => {
                next_state.set(GameState::Encounter);
            }
            (GameState::MainMenu, GameStateTransitionEvent::QuitGame) => {
                evw_exit.send(AppExit);
            }
            (GameState::Encounter, GameStateTransitionEvent::QuitEncounter) => {
                next_state.set(GameState::MainMenu);
            }
            (GameState::GameOver, GameStateTransitionEvent::QuitEncounter) => {
                next_state.set(GameState::MainMenu);
            }
            (GameState::GameOver, GameStateTransitionEvent::StartEncounter) => {
                next_state.set(GameState::Encounter);
            }
            (state_, ev_) => {
                bevy::log::warn!("Got invalid {:?} while in state {:?}", ev_, state_);
            }
        };
    }
}

pub mod state {
    use bevy::prelude::*;

    #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum GameState {
        #[default]
        MainMenu,
        Encounter,
        GameOver,
    }
}
