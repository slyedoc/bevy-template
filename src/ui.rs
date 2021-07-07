use bevy::{
    app::AppExit,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{
    egui::{menu, Checkbox, TopBottomPanel, Ui, Window},
    EguiContext, EguiSettings,
};
use bevy_inspector_egui::{plugin::InspectorWindows, WorldInspectorParams};

use crate::{audio::AudioData, Data};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

#[derive(Inspectable)]
pub struct UIData {
    #[inspectable(min = 0.5, max = 2.0, speed = 0.01, label = "Scale Factor*")]
    scale: f64,

    // window state information
    #[inspectable(ignore)]
    settings: bool,
    #[inspectable(ignore)]
    inspection: bool,
    #[inspectable(ignore)]
    fps: bool,
}

impl Default for UIData {
    fn default() -> Self {
        UIData {
            scale: 1.5,
            settings: false,
            inspection: false,
            fps: false,
        }
    }
}

pub struct UIPlugin;

// This plugin controls a editor like ui
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<UIData>::new().open(true).shared())
            .add_system(update_ui_scale_factor.system())
            .add_system(draw_editor.system());
    }
}

fn update_ui_scale_factor(
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
    data: Res<UIData>,
) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = data.scale / window.scale_factor();
    }
}

fn draw_editor(
    egui_ctx: Res<EguiContext>,
    mut exit: EventWriter<AppExit>,
    mut ui_data: ResMut<UIData>,
    mut world_inspection: ResMut<WorldInspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
    diagnostics: Res<Diagnostics>,
) {
    TopBottomPanel::top("top_panel")
    .min_height(100.0)
    .show(egui_ctx.ctx(), |ui| {
        // The top panel is often a good place for a menu bar:

        menu::bar(ui, |ui| {

            menu::menu(ui, "App", |ui| {
                if ui.button("Quit").clicked() {
                    exit.send(AppExit);
                }
            });

            menu::menu(ui, "Windows", |ui| {
                ui.add(Checkbox::new(&mut world_inspection.enabled, "World"));
                ui.add(Checkbox::new(&mut ui_data.fps, "FPS"));
                draw_menu_item::<Data>(&mut inspector_windows, ui);
                draw_menu_item::<UIData>(&mut inspector_windows, ui);
                draw_menu_item::<AudioData>(&mut inspector_windows, ui);
            });

            menu::menu(ui, "Egui", |ui| {
                ui.add(Checkbox::new(&mut ui_data.settings, "Egui Settings"));
                ui.add(Checkbox::new(&mut ui_data.inspection, "Egui Inspection"));
            });

            let desired_size = ui.available_width();
            ui.add_space(desired_size - 80.0);

            ui.horizontal( |ui| {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(fps_value) = fps.value() {
 
                            ui.label(format!("FPS: {:.2}", fps_value, ));

                    }
                }


            });
        });
    });

    Window::new("Inspection")
        .open(&mut ui_data.inspection)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().inspection_ui(ui);
        });

    Window::new("Settings")
        .open(&mut ui_data.settings)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().settings_ui(ui);
        });

    // setup basic fps window
    Window::new("FPS")
        .open(&mut ui_data.fps)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(fps_value) = fps.value() {
                    if let Some(fps_average) = fps.average() {
                        ui.horizontal(|ui| {
                            ui.label(format!("FPS:"));
                            ui.label(format!("{:.2} (avg {:.2})", fps_value, fps_average));
                        });
                    }
                }
            }

            if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                if let Some(frame_time_value) = frame_time.value() {
                    if let Some(frame_time_avg) = frame_time.average() {
                        ui.horizontal(|ui| {
                            ui.label(format!("frame_time:"));
                            ui.label(format!(
                                "{:.4} (avg {:.4})",
                                frame_time_value, frame_time_avg
                            ));
                        });
                    }
                }
            }

            if let Some(frame_count) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
                if let Some(frame_count_value) = frame_count.value() {
                    ui.horizontal(|ui| {
                        ui.label(format!("frame_count:"));
                        ui.label(format!("{:}", frame_count_value));
                    });
                }
            }
        });
}


fn draw_menu_item<T: 'static>(inspector_windows: &mut ResMut<InspectorWindows>, ui: &mut Ui) {
    let inspector_window_data = inspector_windows.window_data_mut::<T>();
    ui.add(Checkbox::new(
        &mut inspector_window_data.visible,
        &inspector_window_data.name,
    ));
}
