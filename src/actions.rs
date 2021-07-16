use std::fmt;

use bevy::prelude::*;
use bevy_egui::*;
use bevy_input_actionmap::*;
use crate::{editor::{EditorAction, EditorCameraAction}, pong::PongAction, state::StateAction};

// Since I am using action maps I wanted a to display what actions are currently possible
pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {

        // before adding egui multi_threaded drawing like this would fail because
        // to fix ui issues let rest of egui draw first
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(draw_actions.system())
        );
    }
}

pub fn draw_actions(
    egui_ctx: Res<EguiContext>,
    state_map: Res<InputMap<StateAction>>,
    pong_map: Res<InputMap<PongAction>>,
    editor_map: Res<InputMap<EditorAction>>,
    editor_camera_map: Res<InputMap<EditorCameraAction>>,
) {
    egui::Window::new("Key Bindings").show(egui_ctx.ctx(), |ui| {
         dispaly_input_map::<StateAction>(&state_map, ui);
         dispaly_input_map::<PongAction>(&pong_map, ui);
         dispaly_input_map::<EditorAction>(&editor_map, ui);
         dispaly_input_map::<EditorCameraAction>(&editor_camera_map, ui);
    });
}


fn dispaly_input_map<T: 'static  + Send + Sync + fmt::Display>(input_map: &Res<InputMap<T>>, ui: &mut egui::Ui) {
    for (a, b) in &input_map.actions {
        for binding in &b.bindings {
            for key in &binding.keys {
                ui.horizontal(|ui|{
                    ui.set_enabled(false);
                    let _ = ui.button(format!("{:?}", key));
                    ui.label(format!("{} ", a));
                });
            }
            for key in &binding.mouse_buttons {
                ui.horizontal(|ui|{
                    ui.set_enabled(false);
                    let _ = ui.button(format!("Mouse - {:?}", key));
                    ui.label(format!("{} ", a));
                });
            }

            // TODO: Add other button types, like gamepad
        }
     }
}

