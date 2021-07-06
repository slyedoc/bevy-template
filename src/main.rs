mod loading;
mod audio;
mod actions;
mod ui;

use ui::UIPlugin;
use actions::ActionsPlugin;
use loading::LoadingPlugin;
use audio::InternalAudioPlugin;

use bevy::{DefaultPlugins, prelude::{App, ClearColor, Color, Msaa}, window::WindowDescriptor};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorParams, WorldInspectorPlugin, widgets::ResourceInspector};
use bevy_mod_picking::{
    InteractablePickingPlugin, PickingPlugin,
};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};


// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

#[derive(Inspectable, Default)]
pub struct Data {

    clear_color: ResourceInspector<ClearColor>,
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
        .insert_resource(WorldInspectorParams {
            enabled: false,
            despawnable_entities: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(InspectorPlugin::<Data>::new())
        // Add our plugins
        .add_plugin(LoadingPlugin)
        .add_plugin(InternalAudioPlugin)
        .add_plugin(ActionsPlugin)
        .add_plugin(UIPlugin)
        // App State
        .add_state(GameState::Loading);



              app.add_plugin(LogDiagnosticsPlugin::default());

        app.run()
}

