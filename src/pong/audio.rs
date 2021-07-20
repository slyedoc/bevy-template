use super::{PongData, events::{BallBounceEvent, GoalEvent}};
use crate::loading::PongAssets;
use bevy::prelude::*;
use bevy_kira_audio::{Audio};

pub fn handle_audio_events(
    mut ev_ball_bounce: EventReader<BallBounceEvent>,
    mut ev_goal: EventReader<GoalEvent>,
    audio: Res<Audio>,
    assets: Res<PongAssets>,
    data: Res<PongData>,
) {
    for _ in ev_ball_bounce.iter() {
        audio.play_in_channel(assets.pong_bounce.clone(), &data.audio.channel);
    }

    for _ in ev_goal.iter() {
        audio.play_in_channel(assets.pong_goal.clone(), &data.audio.channel );
    }
}

