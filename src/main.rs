mod editor;
mod helpers;
mod loading;
mod menu;
mod pong;
mod state;
mod tic_tac_toe;
mod actions;
use std::fmt;

use actions::ActionsPlugin;
use bevy_egui::EguiPlugin;

use bevy_kira_audio::AudioPlugin;
use editor::EditorPlugin;
use loading::LoadingPlugin;

use bevy::ecs::{archetype::Archetypes, component::Components};
use bevy::prelude::*;
use bevy::reflect::TypeRegistration;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickingEvent};
use menu::MenuPlugin;
use pong::PongPlugin;
use state::StatePlugin;
use strum::EnumIter;
use tic_tac_toe::TicTacToePlugin;

// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    // For now we just load everything
    Loading,
    // Different Games
    Pong,
    TicTacToe,
    // Main Menu
    Menu,
}

// Implement `Display` for `MinMax`.
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        match *self {
            GameState::Loading => write!(f, "Loading"),
            GameState::Pong => write!(f, "Pong"),
            GameState::TicTacToe => write!(f, "Tic-Tack-Toe"),
            GameState::Menu => write!(f, "Menu"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum GameStages {
    Editor, // only used for ui currently
}

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Template".to_string(),
            ..Default::default()
        })

        // Load 3rd party plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(DefaultPickingPlugins)
        // Add States
        .add_state(GameState::Loading)
        // Should send us to Menu once everything is loaded, been having issues with this, maybe media related
        .add_plugin(LoadingPlugin)
        // Add our plugins
        .add_plugin(ActionsPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(StatePlugin)
        .add_plugin(PongPlugin)
        .add_plugin(TicTacToePlugin)
        .add_plugin(MenuPlugin)

        //

        .run()
}



#[allow(dead_code)]
fn print_resources(archetypes: &Archetypes, components: &Components) {
    let mut r: Vec<String> = archetypes
        .resource()
        .components()
        .map(|id| components.get_info(id).unwrap())
        // get_short_name removes the path information
        // i.e. `bevy_audio::audio::Audio` -> `Audio`
        // if you want to see the path info replace
        // `TypeRegistration::get_short_name` with `String::from`
        .map(|info| TypeRegistration::get_short_name(info.name()))
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}

pub fn print_picking_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        println!("This event happened! {:?}", event);
    }
}
