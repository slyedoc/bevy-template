use crate::loading::AudioAssets;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui::*, *};
use bevy_kira_audio::{Audio, AudioChannel};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MenuAudio {
            background: AudioChannel::new("menu-bg".to_owned()),
        })
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(start_audio.system()))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(draw_menu.system()))
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(stop_audio.system()));
    }
}

fn draw_menu(
    egui_ctx: Res<EguiContext>,
    mut exit: EventWriter<AppExit>,
    mut state: ResMut<State<GameState>>,
) {

    SidePanel::left("menu")
        .default_width(200.0)
        .resizable(false)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Bevy Template");
            ui.separator();
            ui.label("Game States");
            if ui.button("Pong").clicked() {
                state.set(GameState::Pong).unwrap();
            }
            if ui.button("Tic-Tac-Toe").clicked() {
                state.set(GameState::TicTacToe).unwrap();
            }
            ui.separator();
            if ui.button("Settings").clicked() {}

            ui.separator();
            if ui.button("Exit").clicked() {
                exit.send(AppExit);
            }
        });
}

struct MenuAudio {
    background: AudioChannel,
}

fn start_audio(audio: Res<Audio>, audio_assets: Res<AudioAssets>, menu: Res<MenuAudio>) {
    println!("play menu sound!");
    audio.play_looped_in_channel(audio_assets.background_music.clone(), &menu.background);
}

fn stop_audio(audio: Res<Audio>, menu: Res<MenuAudio>) {
    audio.pause_channel(&menu.background);
}
