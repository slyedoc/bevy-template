mod loading;
mod audio;
mod editor;
mod game_state;
mod helpers;

use bevy_egui::EguiPlugin;
use editor::EditorPlugin;
use game_state::GameStatePlugin;
use loading::LoadingPlugin;
use audio::InternalAudioPlugin;

use bevy::ecs::{archetype::Archetypes, component::Components};
use bevy::prelude::*;
use bevy::reflect::TypeRegistration;



use bevy_inspector_egui::{Inspectable, InspectorPlugin, widgets::ResourceInspector};
use bevy_mod_picking::{
    InteractablePickingPlugin, PickingPlugin,
};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};


// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Pong,
    TicTackToe,
    // Here the menu is drawn and waiting for player interaction
    Menu,
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
            // TODO: Rename App
            title: "Bevy Template".to_string(),
            ..Default::default()
        })

        // Load 3rd party plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)

        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(InspectorPlugin::<Data>::new().open(false))
        // Add our plugins
        .add_plugin(EditorPlugin)
        .add_plugin(InternalAudioPlugin)
        .add_plugin(GameStatePlugin)
        // Load our asses then load the main menu
        .add_plugin(LoadingPlugin::new().open(GameState::TicTackToe))

        // App State
        .add_state(GameState::Loading);
        //.add_startup_system(print_resources.system());

        //app.add_plugin(LogDiagnosticsPlugin::default());

        app.run()
}

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