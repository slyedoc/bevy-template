use bevy::{app::AppExit, prelude::*};
use bevy_egui::{EguiContext, egui};
use bevy_inspector_egui::{WorldInspectorParams, plugin::InspectorWindows};

use crate::{Data, audio::AudioData};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

#[derive(Inspectable)]
pub struct UIData {
    settings: bool,
    inspection: bool,
}

impl Default for UIData {
    fn default() -> UIData {
        UIData {
            settings: false,
            inspection: false,
        }
    }
}

pub struct UIPlugin;

// This plugin controls a editor like ui
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .init_resource::<UIData>()
        .add_plugin(InspectorPlugin::<UIData>::new())
        .add_startup_system(setup.system())
        .add_system(top_bar.system())
        .add_system(draw_settings.system())
        .add_system(draw_inspection.system());

    }
}

fn setup(mut inspector_windows: ResMut<InspectorWindows>) {

    //hide_window::<Data>(&mut inspector_windows);
    hide_window::<AudioData>(&mut inspector_windows);
}

fn hide_window<T: 'static>(inspector_windows: &mut InspectorWindows) {
    let inspector_window_data = inspector_windows.window_data_mut::<T>();
    inspector_window_data.visible = false;
}


fn top_bar(
    egui_ctx: Res<EguiContext>,
    mut exit: EventWriter<AppExit>,
    mut ui_data: ResMut<UIData>,
    mut world_inspection: ResMut<WorldInspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "App", |ui| {
                if ui.button("Quit").clicked() {
                    exit.send(AppExit);
                }
            });
            egui::menu::menu(ui, "Windows", |ui| {
                ui.add(egui::Checkbox::new(&mut world_inspection.enabled, "World"));
                ui.add(egui::Checkbox::new(&mut ui_data.settings, "Egui Settings"));
                ui.add(egui::Checkbox::new(&mut ui_data.inspection, "Inspection"));

                draw_menu_item::<Data>(&mut inspector_windows,  ui);
                draw_menu_item::<AudioData>(&mut inspector_windows, ui);
            });
        });
    });
}

fn draw_menu_item<T: 'static>(inspector_windows: &mut ResMut<InspectorWindows>, ui: &mut egui::Ui) {
    let inspector_window_data = inspector_windows.window_data_mut::<T>();
    ui.add(egui::Checkbox::new(&mut inspector_window_data.visible, &inspector_window_data.name));
}

fn draw_fps(egui_ctx: Res<EguiContext>, mut ui_state: ResMut<UIData>) {

    
    egui::Window::new("🔧 Inspection")
        .open(&mut ui_state.inspection)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().inspection_ui(ui);
        });
}

fn draw_settings(egui_ctx: Res<EguiContext>, mut ui_state: ResMut<UIData>) {
    egui::Window::new("🔧 Settings")
        .open(&mut ui_state.settings)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().settings_ui(ui);
        });
}

fn draw_inspection(egui_ctx: Res<EguiContext>, mut ui_state: ResMut<UIData>) {
    egui::Window::new("🔧 Inspection")
        .open(&mut ui_state.inspection)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().inspection_ui(ui);
        });
}