use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Tanks)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/bensound-creativeminds.wav")]
    pub background_music: Handle<AudioSource>,

    #[asset(path = "audio/pong/bounce.wav")]
    pub pong_bounce: Handle<AudioSource>,

    #[asset(path = "audio/pong/lowDown.wav")]
    pub pong_goal: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Texture>,
}
