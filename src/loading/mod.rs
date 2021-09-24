use crate::GameState;

use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;


pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {

        // bevy_asset_loader doesnt support loading folders, so we are going to have our
        // own LoadingState will wait for our folder loading and AssetLoader to complete
        AssetLoader::new(GameState::Loading, GameState::Tanks)
            .with_collection::<FontAssets>()
            .with_collection::<MusicAssets>()
            .with_collection::<PongAssets>()
            .build(app);
    }
}


#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct MusicAssets {
    #[asset(path = "audio/bensound-creativeminds.wav")]
    pub creativeminds: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct PongAssets {
    #[asset(path = "audio/pong/bounce.wav")]
    pub pong_bounce: Handle<AudioSource>,

    #[asset(path = "audio/pong/lowDown.wav")]
    pub pong_goal: Handle<AudioSource>,
}
