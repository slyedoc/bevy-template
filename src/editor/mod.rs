pub mod camera;
pub mod grid;
pub mod ui;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
pub use camera::*;
pub use grid::*;
pub use ui::*;

use crate::{GameStages};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EditorState {
    Loading,
    Playing,
    Disabled,
}

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(EditorState::Loading)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            despawnable_entities: false,
            ..Default::default()
        })
        .add_stage_before(
            CoreStage::Update,
            GameStages::Editor,
            SystemStage::parallel(),
        )

        // This is still a bit werid to me, this next like fixs the error, but its not clear to by why
        // the state already exists, but this works it works
        // see https://github.com/bevyengine/bevy/issues/2312
        .add_state_to_stage(GameStages::Editor, EditorState::Playing)

        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CameraPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(UIPlugin)
        .add_system(handle_keyboard.system())
        .add_system_set(SystemSet::on_update(EditorState::Loading).with_system(loaded.system()));
    }
}

fn handle_keyboard(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<EditorState>>) {
    if keyboard_input.just_pressed(KeyCode::F12) {
        let result = match state.current() {
            // could only happen if loading takes a while for frist frame, but go ahead and disable editor if so
            EditorState::Loading => EditorState::Disabled,
            EditorState::Playing => EditorState::Disabled,
            EditorState::Disabled => EditorState::Loading,
        };
        state.set(result).expect("Editor state didnt set");
    }
}

// first frame after 'Loading', we set state to 'Playing'
fn loaded(mut state: ResMut<State<EditorState>>) {
    if state.current().eq(&EditorState::Loading) {
        state
            .set(EditorState::Playing)
            .expect("Editor state didnt set in loaded.");
    }
}

#[allow(dead_code)]
pub fn run_if_editor(state: Res<State<EditorState>>) -> ShouldRun {
    match state.current() {
        EditorState::Loading => ShouldRun::Yes,
        EditorState::Playing => ShouldRun::Yes,
        EditorState::Disabled => ShouldRun::No,
    }
}
