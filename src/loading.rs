use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::{AudioSource};


pub struct LoadingPlugin {
    target: GameState,
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

impl LoadingPlugin {
    /// Creates a new inspector plugin with access to `World` in the [`Context`](crate::Context).
    pub fn new() -> Self {
        LoadingPlugin {
            target: GameState::Menu,
        }
    }
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

    //https://www.bensound.com/royalty-free-music/track/creative-minds
    #[asset(path = "audio/bensound-creativeminds.mp3")]
    pub background_music: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Texture>,
}
