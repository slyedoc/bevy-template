use bevy::app::AppExit;
use bevy::prelude::*;

use bevy_egui::{*, egui::{*}};

use crate::GameState;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(draw_menu.system()));
    }
}

fn draw_menu(
    egui_ctx: Res<EguiContext>,
    mut exit: EventWriter<AppExit>,
    mut state: ResMut<State<GameState>>,
) {
    SidePanel::left( "menu" )
        .default_width(200.0)
        .resizable(false)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Bevy Template");

            if ui.button("Pong").clicked() {
                state.set(GameState::Pong).unwrap();
            }

            if ui.button("Tic-Tac-Toe").clicked() {
                state.set(GameState::TicTacToe).unwrap();
            }

            if ui.button("Exit").clicked() {
                exit.send(AppExit);
            }
        });
}
