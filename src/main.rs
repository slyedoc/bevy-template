mod audio;
mod editor;
mod game_state;
mod helpers;
mod loading;

use std::fmt;

use audio::InternalAudioPlugin;
use bevy_egui::EguiPlugin;
use editor::EditorPlugin;
use game_state::GameStatePlugin;
use loading::LoadingPlugin;

use bevy::ecs::{archetype::Archetypes, component::Components};
use bevy::prelude::*;
use bevy::reflect::TypeRegistration;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_inspector_egui::{widgets::ResourceInspector, Inspectable, InspectorPlugin};
use bevy_mod_picking::{DefaultPickingPlugins, PickingEvent};
use strum::EnumIter;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
pub enum GameStages {
    Editor, // only used for ui currently
}

#[derive(Inspectable, Default)]
pub struct Data {
    #[inspectable(label = "Background Color")]
    clear_color: ResourceInspector<ClearColor>,
    ui: ResourceInspector<editor::ui::UIData>,
    #[inspectable(min = 1169)]
    seed: u64,
}

fn main() {
    let mut app = App::build();
    app

        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            vsync: true,
            resizable: true,
            title: "Bevy Template".to_string(),
            ..Default::default()
        })

        // Load 3rd party plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)


        .add_plugin(DefaultPickingPlugins)
        .add_plugin(InspectorPlugin::<Data>::new().open(false))
        // Add our plugins
        .add_plugin(EditorPlugin)
        .add_plugin(InternalAudioPlugin)
        .add_plugin(GameStatePlugin)
        // Load our asses then load the main menu
        .add_plugin(LoadingPlugin::new().open(GameState::Menu))

        // App State
        .add_state(GameState::Loading)
        //.add_startup_system(print_resources.system());
        .add_system_to_stage(CoreStage::PostUpdate, print_picking_events.system())

        //app.add_plugin(LogDiagnosticsPlugin::default());
        ;
    app.run()
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
