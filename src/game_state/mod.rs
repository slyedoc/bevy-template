use bevy::{app::AppExit, prelude::*};
use bevy_input_actionmap::*;
mod menu;
mod pong;
mod tick_tack_toe;

pub use menu::*;
pub use pong::*;
pub use tick_tack_toe::*;

use crate::GameState;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(ActionPlugin::<Action>::default())
        .add_plugin(PongPlugin)
        .add_plugin(TicTackToePlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(setup.system())
        .add_system(actions.system());
    }
}


#[derive(Hash, PartialEq, Eq, Clone)]
enum Action {
    Back,
    Pong,
    TicTackToe,
}

fn setup(mut input: ResMut<InputMap<Action>>) {
    input
        .bind(Action::Back, KeyCode::Escape)
        .bind(Action::Pong, KeyCode::F1)
        .bind(Action::TicTackToe, KeyCode::F2)
        ;
}

fn actions(mut state: ResMut<State<GameState>>, input: Res<InputMap<Action>>, mut app_exit: EventWriter<AppExit>) {

    if input.just_active(Action::Back) {
        if state.current().eq(&GameState::Menu) {
            app_exit.send(AppExit);
        } else {
            state.set(GameState::Menu).unwrap();
        }
    }


    set_state(Action::Pong, GameState::Pong, &input, &mut state);
    set_state(Action::TicTackToe, GameState::TicTackToe, &input, &mut state);
}

// Sets the GameState if its not already set
fn set_state(action: Action, target: GameState, input: &Res<InputMap<Action>>, state: &mut ResMut<State<GameState>>) {
    if input.just_active(action) {
        if !state.current().eq(&target) {
            state.set(target).unwrap();
        }
    }
}