//use crate::actions::Actions;
use crate::{loading::AudioAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

use bevy_inspector_egui::{Inspectable, InspectorPlugin};

#[derive(Inspectable, Default)]
pub struct AudioData {

    #[inspectable(min = 0.5, max = 1.0)]
    volume: f32,
    muted: bool,
    #[inspectable(min = 1169)]
    seed: u64,
}

pub struct InternalAudioPlugin;

// This plugin is responsible to controll the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .init_resource::<AudioData>()
            .add_plugin(InspectorPlugin::<AudioData>::new())
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(start_audio.system()),
            );
            // .add_system_set(
            //     SystemSet::on_update(GameState::Playing).with_system(control_flying_sound.system()),
            // );
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, data: Res<AudioData>) {
    println!("start");
    audio.set_volume(data.volume);
    audio.play_looped(audio_assets.background_music.clone());
}
