
use bevy_kira_audio::{AudioChannel};
use bevy_inspector_egui::{ Inspectable};


#[derive(Inspectable, Default, Debug)]
pub struct AudioState {
    pub state: ChannelAudioState,    
    #[inspectable(ignore)]
    pub channel: AudioChannel,
}

#[derive(Inspectable, Debug)]
pub struct ChannelAudioState {
    stopped: bool,
    paused: bool,
    loop_started: bool,
    volume: f32,
}

impl Default for ChannelAudioState {
    fn default() -> Self {
        ChannelAudioState {
            volume: 1.0,
            stopped: true,
            loop_started: false,
            paused: false,
        }
    }
}