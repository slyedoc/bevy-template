use super::events::{BallBounceEvent, GoalEvent};
use crate::loading::AudioAssets;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};

pub struct PongAudio {
    channel_background: AudioChannel,
    #[allow(dead_code)]
    channel_bounce: AudioChannel,
    channel_goal: AudioChannel,
}

impl Default for PongAudio {
    fn default() -> Self {
        PongAudio {
            channel_background: AudioChannel::new("pong-bg".to_owned()),
            channel_bounce: AudioChannel::new("pong-fx1".to_owned()),
            channel_goal: AudioChannel::new("pong-fx2".to_owned()),
        }
    }
}

pub fn start_bg_audio(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    audio_state: Res<PongAudio>,
) {
    println!("play pong background sound!");
    audio.play_in_channel(audio_assets.pong_bounce.clone(), &audio_state.channel_background); //
}

pub fn stop_bg_audio(audio: Res<Audio>, audio_state: Res<PongAudio>) {
    audio.stop_channel(&audio_state.channel_background);
}

pub fn handle_audio_events(
    mut ev_ball_bounce: EventReader<BallBounceEvent>,
    mut ev_goal: EventReader<GoalEvent>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    audio_state: Res<PongAudio>,
    assets: Res<AssetServer>,
) {
    for _ in ev_ball_bounce.iter() {
        println!("play bounce");
        let channel = AudioChannel::new("pong-bounce-ball".to_owned());
        audio.play_in_channel(assets.load("audio/pong/bounce.wav"), &channel); // works
        audio.play_in_channel(audio_assets.pong_bounce.clone(), &channel); // NOTHING
    }

    for _ in ev_goal.iter() {
        println!("play goal");
        audio.play_looped_in_channel(audio_assets.pong_goal.clone(), &audio_state.channel_goal); // NOTHING
    }
}
