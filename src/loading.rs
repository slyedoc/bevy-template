use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::{AudioSource};

pub struct LoadingPlugin {
    target: GameState,
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)
#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/bensound-creativeminds.wav")]
    pub background_music: Handle<AudioSource>,

    #[asset(path = "audio/bensound-onceagain.wav")]
    pub background_music2: Handle<AudioSource>,

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


/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, self.target.clone())
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

impl LoadingPlugin {
    pub fn open(self, state: GameState) -> Self {
        LoadingPlugin {
            target: state,
            ..self
        }
    }
}

impl Default for LoadingPlugin {
    fn default() -> Self {
        LoadingPlugin {
            target: GameState::Menu,
        }
    }
}

