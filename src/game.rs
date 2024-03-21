use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemSet {
    Global,
    MainMenu,
    GameOver,
}

pub mod transition {
    use bevy::app::AppExit;
    use bevy::prelude::*;

    use super::state::PauseState;
    use super::state::State as GameState;
    use super::SystemSet;

    #[derive(Event, Debug)]
    pub enum Event {
        Play,
        TogglePause,
        GameOver,
        QuitToMainMenu,
        Quit,
    }

    pub struct StateTransition;

    impl Plugin for StateTransition {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, handle_transition.in_set(SystemSet::Global))
                .add_event::<Event>();
        }
    }

    fn handle_transition(
        state: Res<State<GameState>>,
        mut evr_transition: EventReader<Event>,
        mut next_state: ResMut<NextState<GameState>>,
        mut evw_exit: EventWriter<AppExit>,
    ) {
        let state = state.get();
        let Some(ev) = evr_transition.read().next() else {
            return;
        };
        match (state, ev) {
            (GameState::MainMenu, Event::Play) => {
                next_state.set(GameState::InGame(super::state::PauseState::Running));
            }
            (GameState::MainMenu, Event::Quit) => {
                evw_exit.send(AppExit);
            }
            (GameState::InGame(PauseState::Running), Event::TogglePause) => {
                next_state.set(GameState::InGame(super::state::PauseState::Pause));
            }
            (GameState::InGame(PauseState::Pause), Event::TogglePause) => {
                next_state.set(GameState::InGame(super::state::PauseState::Running));
            }
            (GameState::GameOver, Event::QuitToMainMenu) => {
                next_state.set(GameState::MainMenu);
            }
            (GameState::GameOver, Event::Play) => {
                next_state.set(GameState::InGame(PauseState::Running));
            }
            (state_, ev_) => {
                bevy::log::warn!("Got {:?} while in state {:?}", ev_, state_);
            }
        };
    }
}

pub mod state {
    use bevy::prelude::*;

    #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum State {
        #[default]
        MainMenu,
        InGame(PauseState),
        GameOver,
    }

    #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum PauseState {
        #[default]
        Running,
        Pause,
    }
}
