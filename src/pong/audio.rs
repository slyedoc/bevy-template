use crate::loading::AudioAssets;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};

use super::events::{BallBounceEvent, GoalEvent};

pub struct PongAudio {
    background: AudioChannel,
    bounce: AudioChannel,
    goal: AudioChannel,
}

impl Default for PongAudio {
    fn default() -> Self {
        PongAudio {
            background: AudioChannel::new("pong-bg".to_owned()),
            bounce: AudioChannel::new("pong-fx1".to_owned()),
            goal: AudioChannel::new("pong-fx2".to_owned()),
        }
    }
}


pub fn start_bg_audio(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    audio_state: Res<PongAudio>,
) {

    println!("play pong background sound!");
    audio.play_looped_in_channel(
        audio_assets.background_music2.clone(),
        &audio_state.background,
    );
}

pub fn stop_bg_audio(audio: Res<Audio>, audio_state: Res<PongAudio>) {
    audio.stop_channel(&audio_state.background);
}

pub fn handle_audio_events(
    mut ev_ball_bounce: EventReader<BallBounceEvent>,
    mut ev_goal: EventReader<GoalEvent>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    audio_state: Res<PongAudio>,
) {

    for _ in ev_ball_bounce.iter() {
        println!("play bounce");
        let channel = AudioChannel::new("pong-bounce-ball".to_owned());
        audio.play_in_channel(audio_assets.pong_bounce.clone(), &channel);

    }

    for _ in ev_goal.iter() {
        println!("play goal");
        audio.play_looped_in_channel(audio_assets.pong_goal.clone(), &audio_state.goal);
    }
}
