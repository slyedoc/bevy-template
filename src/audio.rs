//use crate::actions::Actions;
use crate::{loading::AudioAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

use bevy_inspector_egui::{Inspectable, InspectorPlugin};

#[derive(Inspectable)]
pub struct AudioData {

    #[inspectable(min = 0.5, max = 1.0)]
    volume: f32,
    muted: bool,
}

impl Default for AudioData {
    fn default() -> Self {
        Self {
            volume: 1.0,
            muted: false,
        }
    }
}

pub struct InternalAudioPlugin;

// This plugin is responsible to controll the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_plugin(InspectorPlugin::<AudioData>::new().open(false))
            .add_system_set(
                SystemSet::on_enter(GameState::Menu).with_system(start_audio.system()),
            );
            // .add_system_set(
            //     SystemSet::on_update(GameState::Playing).with_system(control_flying_sound.system()),
            // );
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, data: Res<AudioData>) {

    println!("audio");
    audio.set_volume(data.volume);
    audio.play_looped(audio_assets.background_music.clone());
}
