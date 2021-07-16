use crate::GameState;
use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader, LoadingAssetHandles};
use bevy_kira_audio::AudioSource;
use std::fmt::Debug;
use std::hash::Hash;

// Currently AssetLoader doesnt work without release mode
// Has something to do with bevy_assets and strong handles
// See https://github.com/bevyengine/bevy/issues/2347
// NiklasEi created bevy_asset_loader
// That same day he made changes to address it
// See https://github.com/NiklasEi/bevy_asset_loader/commit/c36bb6072069b9577c41f1e1b5724594b0465b01
pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);

        app.add_system_set(
            SystemSet::on_update(GameState::Loading)
            //.with_system(display_loading_status::<Handle<Font>, FontAssets>.system())
            //.with_system(display_loading_status::<Handle<AudioSource>, AudioAssets>.system())
            //.with_system(display_loading_status::<Handle<Texture>, AudioAssets>.system())
        );
    }
}

// AssetLoader is not going to menu, stuck in loading state
#[allow(dead_code)]
fn display_loading_status<T: Component + Debug + Clone + Eq + Hash, Assets: AssetCollection>(
    asset_server: Res<AssetServer>,
    loading_asset_handles: Option<Res<LoadingAssetHandles<Assets>>>,
) {
    if let Some(loading_asset_handles) = loading_asset_handles {
        let load_state = asset_server.get_group_load_state(loading_asset_handles.handles.clone());        
        println!("{:?} - {:?}", std::any::type_name::<T>(), load_state);
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
    #[asset(path = "audio/bensound-creativeminds.wav")]
    pub background_music: Handle<AudioSource>,

    // #[asset(path = "audio/bensound-onceagain.wav")]
    // pub background_music2: Handle<AudioSource>,

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
