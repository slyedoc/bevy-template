use crate::GameState;
use bevy::{app::AppExit, prelude::*};
use bevy_input_actionmap::*;
use std::fmt;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ActionPlugin::<StateAction>::default())
            .add_startup_system(setup.system())
            .add_system(run_actions.system())
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_exit.system()))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(setup_back.system()))
            .add_system_set(SystemSet::on_enter(GameState::Pong).with_system(pong_enter.system()))
            .add_system_set(SystemSet::on_exit(GameState::Pong).with_system(pong_exit.system()))
            .add_system_set(SystemSet::on_enter(GameState::TicTacToe).with_system(tic_tac_toe_enter.system()))
            .add_system_set(SystemSet::on_exit(GameState::TicTacToe).with_system(tic_tac_toe_exit.system()));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum StateAction {
    Exit,
    Back,
    Pong,
    TicTacToe,
}
impl fmt::Display for StateAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateAction::Exit => write!(f, "Exit Application"),
            StateAction::Back => write!(f, "Return to Main Menu"),
            StateAction::Pong => write!(f, "Launch Pong"),
            StateAction::TicTacToe => write!(f, "Launch Tic-Tac-Toe"),
        }
    }
}

fn setup(mut input: ResMut<InputMap<StateAction>>) {
    input
        .bind(StateAction::Pong, KeyCode::F1)
        .bind(StateAction::TicTacToe, KeyCode::F2);
}

fn setup_exit(mut input: ResMut<InputMap<StateAction>>) {
    input
        .remove_action(StateAction::Back)
        .bind(StateAction::Exit, KeyCode::Escape);
}

fn setup_back(mut input: ResMut<InputMap<StateAction>>) {
    input
        .remove_action(StateAction::Exit)
        .bind(StateAction::Back, KeyCode::Escape);
}

fn pong_enter(mut input: ResMut<InputMap<StateAction>>) {
    input.remove_action(StateAction::Pong);
}

fn pong_exit(mut input: ResMut<InputMap<StateAction>>) {
    input.bind(StateAction::Pong, KeyCode::F1);
}


fn tic_tac_toe_enter(mut input: ResMut<InputMap<StateAction>>) {
    input.remove_action(StateAction::TicTacToe);
}

fn tic_tac_toe_exit(mut input: ResMut<InputMap<StateAction>>) {
    input.bind(StateAction::TicTacToe, KeyCode::F2);
}


fn run_actions(
    mut state: ResMut<State<GameState>>,
    input: Res<InputMap<StateAction>>,
    mut app_exit: EventWriter<AppExit>,
) {
    if input.just_active(StateAction::Exit) {
        app_exit.send(AppExit);
    }

    set_state(StateAction::Back, GameState::Menu, &input, &mut state);
    set_state(StateAction::Pong, GameState::Pong, &input, &mut state);
    set_state(
        StateAction::TicTacToe,
        GameState::TicTacToe,
        &input,
        &mut state,
    );
}
fn set_state(
    action: StateAction,
    target: GameState,
    input_map: &Res<InputMap<StateAction>>,
    state: &mut ResMut<State<GameState>>,
) {
    if input_map.just_active(action) {
        if !state.current().eq(&target) {
            state.set(target).unwrap();
        }
    }
}
