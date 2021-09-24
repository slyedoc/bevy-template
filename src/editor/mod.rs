pub mod camera;
pub mod grid;
pub mod ui;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_input_actionmap::{ActionPlugin, InputMap};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use std::fmt;
pub use camera::*;
pub use grid::*;
pub use ui::*;

use crate::{GameStages, actions::ActionsWindow};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EditorState {
    Loading,
    Playing,
    Disabled,
}

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WorldInspectorParams {
            enabled: false,
            despawnable_entities: false,
            ..Default::default()
        })
        .add_plugin(ActionPlugin::<EditorAction>::default())
        .add_state(EditorState::Disabled)
        .add_stage_before(
            CoreStage::Update,
            GameStages::Editor,
            SystemStage::parallel(),
        )
        // This is still a bit werid to me, this next like fixs the error, but its not clear to by why
        // the state already exists, but this works it works
        // see https://github.com/bevyengine/bevy/issues/2312
        .add_state_to_stage(GameStages::Editor, EditorState::Disabled)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CameraPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup.system())
        .add_system(run_actions.system())
        .add_system_set(SystemSet::on_update(EditorState::Loading).with_system(loaded.system()));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum EditorAction {
    Editor,
    World,
    Keys,
}

impl fmt::Display for EditorAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorAction::Editor => write!(f, "Toggle Editor"),
            EditorAction::World => write!(f, "Open World Inspector"),
            EditorAction::Keys => write!(f, "Toggle Key Mappings"),
        }
    }
}

fn setup(mut input_map: ResMut<InputMap<EditorAction>>) {
    input_map.bind(EditorAction::Editor, KeyCode::F12);
    input_map.bind(EditorAction::World, KeyCode::F11);
    input_map.bind(EditorAction::Keys, KeyCode::F10);
}

fn run_actions(
    input_map: Res<InputMap<EditorAction>>,
     mut state: ResMut<State<EditorState>>,
     mut world_inspection: ResMut<WorldInspectorParams>,
     mut action_window: ResMut<ActionsWindow>,
    ) {
    if input_map.just_active(EditorAction::Editor) {
        let result = match state.current() {
            // could only happen if loading takes a while for frist frame, but go ahead and disable editor if so
            EditorState::Loading => EditorState::Disabled,
            EditorState::Playing => EditorState::Disabled,
            EditorState::Disabled => EditorState::Loading,
        };
        state.set(result).expect("Editor state didn't set");
    }

    if input_map.just_active(EditorAction::World) {
        world_inspection.enabled = !world_inspection.enabled;
    }

    if input_map.just_active(EditorAction::Keys) {
        action_window.enabled = !action_window.enabled;
    }
}

// first frame after 'Loading', we set state to 'Playing'
fn loaded(mut state: ResMut<State<EditorState>>) {
    if state.current().eq(&EditorState::Loading) {
        state
            .set(EditorState::Playing)
            .expect("Editor state didn't set in loaded.");
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
