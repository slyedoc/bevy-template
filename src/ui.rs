use bevy::{app::AppExit, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, prelude::*};
use bevy_egui::{EguiContext, EguiSettings, egui::{menu, Ui, Checkbox, Color32, TopBottomPanel, Stroke, Window as Window}};
use bevy_inspector_egui::{WorldInspectorParams,  plugin::InspectorWindows};
use emath::*;

use crate::{Data, audio::AudioData};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

#[derive(Inspectable, Default)]
pub struct UIData {
    settings: bool,
    inspection: bool,
    fps: bool,
}

pub struct UIPlugin;

// This plugin controls a editor like ui
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .init_resource::<UIData>()
        .add_plugin(InspectorPlugin::<UIData>::new())
        .add_startup_system(setup.system())
        .add_system(update_ui_scale_factor.system())
        .add_system(draw_top_bar.system())
        .add_system(draw_fps.system())
        .add_system(draw_settings.system())
        .add_system(draw_inspection.system());

    }
}

fn setup(mut inspector_windows: ResMut<InspectorWindows>) {

    //hide_window::<Data>(&mut inspector_windows);
    hide_window::<AudioData>(&mut inspector_windows);
}

fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.5 / window.scale_factor();
    }
}

fn hide_window<T: 'static>(inspector_windows: &mut InspectorWindows) {
    let inspector_window_data = inspector_windows.window_data_mut::<T>();
    inspector_window_data.visible = false;
}


fn draw_top_bar(
    egui_ctx: Res<EguiContext>,
    mut exit: EventWriter<AppExit>,
    mut ui_data: ResMut<UIData>,
    mut world_inspection: ResMut<WorldInspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        // The top panel is often a good place for a menu bar:


        menu::bar(ui, |ui| {

            // let mut style = (**ui.style()).clone();
            // style.spacing.item_spacing = vec2(10.0, 10.0);
            // style.spacing.button_padding = vec2(2.0, 0.0);
            // // style.visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
            // style.visuals.widgets.active.bg_stroke = Stroke::none();
            // // style.visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
            // style.visuals.widgets.hovered.bg_stroke = Stroke::none();
            // style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
            // style.visuals.widgets.inactive.bg_stroke = Stroke::none();
            // ui.set_style(style);

            menu::menu(ui, "App", |ui| {
                if ui.button("Quit").clicked() {
                    exit.send(AppExit);
                }
            });

            menu::menu(ui, "Windows", |ui| {
                ui.add(Checkbox::new(&mut world_inspection.enabled, "World"));
                ui.add(Checkbox::new(&mut ui_data.fps, "FPS"));
                draw_menu_item::<Data>(&mut inspector_windows,  ui);
                draw_menu_item::<AudioData>(&mut inspector_windows, ui);
            });

            menu::menu(ui, "Egui", |ui| {
                ui.add(Checkbox::new(&mut ui_data.settings, "Egui Settings"));
                ui.add(Checkbox::new(&mut ui_data.inspection, "Egui Inspection"));
            });

        });
    });
}

fn draw_menu_item<T: 'static>(inspector_windows: &mut ResMut<InspectorWindows>, ui: &mut Ui) {
    let inspector_window_data = inspector_windows.window_data_mut::<T>();
    ui.add(Checkbox::new(&mut inspector_window_data.visible, &inspector_window_data.name));
}

fn draw_fps(egui_ctx: Res<EguiContext>, mut ui_state: ResMut<UIData>, diagnostics: Res<Diagnostics>) {

    Window::new("FPS")
        .open(&mut ui_state.fps)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {

            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                        ui.label(format!("{:.2}", average));
                }
            }
        });
}

fn draw_settings(egui_ctx: Res<EguiContext>, mut ui_state: ResMut<UIData>) {
    Window::new("Settings")
        .open(&mut ui_state.settings)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().settings_ui(ui);
        });
}

fn draw_inspection(egui_ctx: Res<EguiContext>, mut ui_state: ResMut<UIData>) {
    Window::new("Inspection")
        .open(&mut ui_state.inspection)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().inspection_ui(ui);
        });
}