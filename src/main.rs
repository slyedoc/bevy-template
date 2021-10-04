mod actions;
mod audio;
mod editor;
mod helpers;
mod loading;
mod menu;
mod pong;
mod state;
mod tanks;
mod window_config;

use actions::ActionsPlugin;
use bevy_egui::EguiPlugin;

use std::env::var;
use std::fmt;
use bevy_prototype_debug_lines::DebugLinesPlugin;

use bevy_kira_audio::AudioPlugin;
use editor::EditorPlugin;
use loading::LoadingPlugin;

use bevy::ecs::{archetype::Archetypes, component::Components};
use bevy::prelude::*;
use bevy::reflect::TypeRegistration;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickingEvent};
use convert_case::{Case, Casing};
use menu::MenuPlugin;
use pong::PongPlugin;
use state::StatePlugin;
use strum::EnumIter;
use tanks::TanksPlugin;
use window_config::WindowConfigPlugin;

// See https://bevy-cheatbook.github.io/ for about everything

#[derive(Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
pub enum GameState {
    Loading, // Asset Loading
    Menu,    // Main Menu

    // Different Games
    Pong,
    Tanks,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameState::Loading => write!(f, "Loading"),
            GameState::Pong => write!(f, "Pong"),
            GameState::Tanks => write!(f, "Tanks"),
            GameState::Menu => write!(f, "Menu"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum GameStages {
    Editor, // only used for ui currently
}


fn main() {
    let name = "Bevy Slyedoc Template".to_string();
    let config_home = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
        .unwrap();

    let config_path = format!("{}/{}", config_home, name.to_case(Case::Snake));
    println!("configs: {:?}", config_path);

    let mut app = App::build();

    app.insert_resource(Msaa { samples: 8 })
        .insert_resource(ConfigPath { path: config_path })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: name,
            ..Default::default()
        })
        // Load 3rd party plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DebugLinesPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(DefaultPickingPlugins)

        // Add States
        .add_state(GameState::Loading)
        // Load Assets, and once Load start
        .add_plugin(LoadingPlugin)
        // Add our plugins
        .add_plugin(EditorPlugin)
        .add_plugin(WindowConfigPlugin)
        .add_plugin(ActionsPlugin)
        .add_plugin(StatePlugin)
        .add_plugin(PongPlugin)
        .add_plugin(TanksPlugin::new(GameState::Tanks))
        .add_plugin(MenuPlugin)
        .run();
}

#[allow(dead_code)]
fn print_resources(archetypes: &Archetypes, components: &Components) {
    let mut r: Vec<String> = archetypes
        .resource()
        .components()
        .map(|id| components.get_info(id).unwrap())
        .map(|info| TypeRegistration::get_short_name(info.name()))
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}

#[allow(dead_code)]
pub fn print_picking_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        println!("This event happened! {:?}", event);
    }
}
