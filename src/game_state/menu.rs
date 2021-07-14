use bevy::app::AppExit;
use bevy::prelude::*;

use bevy_egui::*;
use bevy_inspector_egui::egui::emath::{RectTransform, Rect};
use bevy_inspector_egui::egui::{Color32, Pos2, Shape, Stroke, pos2, vec2};

use crate::GameState;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(GameState::Menu)
        .with_system(draw_menu.system()));
    }
}

fn draw_menu(egui_ctx: Res<EguiContext>, mut exit: EventWriter<AppExit>) {
    egui::Window::new("my_area")
        .fixed_pos(egui::pos2(32.0, 32.0))
        .show(egui_ctx.ctx(), |ui| {
            ui.ctx().request_repaint();
            let time = ui.input().time;

            let desired_size = ui.available_width() * vec2(1.0, 0.35);
            let (_id, rect) = ui.allocate_space(desired_size);

            let to_screen =
                RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

            let mut shapes = vec![];

            for &mode in &[2, 3, 5] {
                let mode = mode as f32;
                let n = 120;
                let speed = 1.5;

                let points: Vec<Pos2> = (0..=n)
                    .map(|i| {
                        let t = i as f32 / (n as f32);
                        let amp = (time as f32 * speed * mode).sin() / mode;
                        let y = amp * (t * std::f32::consts::TAU / 2.0 * mode).sin();
                        to_screen * pos2(t, y)
                    })
                    .collect();

                let thickness = 10.0 / mode;
                shapes.push(Shape::line(
                    points,
                    Stroke::new(thickness, Color32::from_additive_luminance(196)),
                ));
            }

            ui.painter().extend(shapes);
        });

        egui::Area::new("exit")
        .fixed_pos(egui::pos2(20.0, 300.0))
        .show(egui_ctx.ctx(), |ui| {
            if ui.button("Exit").clicked() {
                exit.send(AppExit);
            }
        });
}
