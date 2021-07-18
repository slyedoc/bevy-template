use crate::GameState;
use bevy::{app::AppExit, prelude::*};
use bevy_input_actionmap::*;
use std::fmt;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ActionPlugin::<StateAction>::default())
            .add_startup_system(setup.system())
            .add_system(run_actions.system());
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum StateAction {
    Exit,
    Game(GameState)
}


impl fmt::Display for StateAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateAction::Exit => write!(f, "Exit Application"),
            StateAction::Game(g) => {
                match g {
                    GameState::Menu => write!(f, "Back to menu"),
                    _ => write!(f, "Lanuch {}", g)
                }
            }
        }
    }
}

fn setup(mut input: ResMut<InputMap<StateAction>>) {
    input
        .bind( StateAction::Game(GameState::Menu), KeyCode::Escape)
        .bind( StateAction::Game(GameState::Tanks), KeyCode::F1)
        .bind( StateAction::Game(GameState::Pong), KeyCode::F2);
}


fn run_actions(
    mut state: ResMut<State<GameState>>,
    input: Res<InputMap<StateAction>>,
    mut app_exit: EventWriter<AppExit>,
) {
    if input.just_active(StateAction::Exit) {
        app_exit.send(AppExit);
    }

    if input.just_active(StateAction::Game(GameState::Pong)) {
        if *state.current() != GameState::Pong {
            state.set(GameState::Pong).unwrap();
        }
    }

    if input.just_active(StateAction::Game(GameState::Tanks)) {
        if *state.current() != GameState::Tanks {
            state.set(GameState::Tanks).unwrap();
        }
    }

    if input.just_active(StateAction::Game(GameState::Menu)) {
        if *state.current() != GameState::Menu {
            state.set(GameState::Menu).unwrap();
        }
    }
}
