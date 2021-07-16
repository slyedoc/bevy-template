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

use crate::{GameStages, GameState, pong::PongData, tick_tac_toe::TicTacToeData};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

use super::{grid::GridData, EditorCamera, EditorState};
use strum::IntoEnumIterator;

#[derive(Inspectable)]
pub struct UIData {
    #[inspectable(
        min = 0.5,
        max = 2.0,
        speed = 0.01,
        label = "Scale Factor *warning* - may panic"
    )]
    scale: f64,

    camera: EditorCamera,

    // Window Information
    #[inspectable(ignore)]
    fps: bool,

    // egui information
    #[inspectable(ignore)]
    egui_settings: bool,
    #[inspectable(ignore)]
    egui_inspection: bool,
}

impl Default for UIData {
    fn default() -> Self {
        UIData {
            scale: 1.5,
            camera: EditorCamera::Perspective,
            egui_settings: false,
            egui_inspection: false,
            fps: false,
        }
    }
}

pub struct UIPlugin;

// This plugin controls a editor like ui
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<UIData>::new().open(false).shared())
            .add_system(update_ui_scale_factor.system())
            .add_system_set_to_stage(
                GameStages::Editor,
                SystemSet::on_update(EditorState::Playing).with_system(draw_editor_topbar.system()),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
struct EditorUIStage;

// TODO: Change this value to much and egui panics
// This runs all the time, to update the scale factor for any ui the game has
fn update_ui_scale_factor(
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
    data: Res<UIData>,
) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = data.scale / window.scale_factor();
    }
}

fn draw_editor_topbar(
    egui_ctx: Res<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut ui_data: ResMut<UIData>,
    mut world_inspection: ResMut<WorldInspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
    diagnostics: Res<Diagnostics>,
) {
    TopBottomPanel::top("top_panel")
        .min_height(100.0)
        .show(egui_ctx.ctx(), |ui| {
            menu::bar(ui, |ui| {
                menu::menu(ui, "App", |ui| {
                    if ui.button("Quit").clicked() {
                        exit.send(AppExit);
                    }
                });

                menu::menu(ui, "Windows", |ui| {
                    ui.add(Checkbox::new(&mut world_inspection.enabled, "World"));
                    ui.add(Checkbox::new(&mut ui_data.fps, "FPS"));
                });

                menu::menu(ui, "Resources", |ui| {
                    draw_menu_item::<TicTacToeData>(&mut inspector_windows, ui);
                    draw_menu_item::<PongData>(&mut inspector_windows, ui);
                    draw_menu_item::<UIData>(&mut inspector_windows, ui);
                    draw_menu_item::<GridData>(&mut inspector_windows, ui);
                });

                menu::menu(ui, "Egui", |ui| {
                    ui.add(Checkbox::new(&mut ui_data.egui_settings, "Egui Settings"));
                    ui.add(Checkbox::new(
                        &mut ui_data.egui_inspection,
                        "Egui Inspection",
                    ));
                });

                // TODO: Figure out better way to align right
                let desired_size = ui.available_width();
                ui.add_space(desired_size - 200.0);

                menu::menu(ui, format!("State: {}", state.current()), |ui| {
                    for s in GameState::iter() {
                        match s {
                            GameState::Loading => {}
                            _ => {
                                // Don't set state to current state, will panic
                                if s != *state.current() {
                                    if ui.button(format!("{}", s)).clicked() {
                                        state.set(s).unwrap();
                                    }
                                } else {
                                    ui.label(format!("{}", s));
                                }
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(fps_value) = fps.value() {
                            ui.label(format!("FPS: {:.0}", fps_value,));
                        }
                    }
                });
            });
        });

    Window::new("Inspection")
        .open(&mut ui_data.egui_inspection)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().inspection_ui(ui);
        });

    Window::new("Settings")
        .open(&mut ui_data.egui_settings)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().settings_ui(ui);
        });

    // setup basic fps window, could use some plots and far better code, but ok for now
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
