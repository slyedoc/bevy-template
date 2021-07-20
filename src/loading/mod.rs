mod inspector_altas;
mod dungeon_pack;

use crate::GameState;

use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_asset_ron::RonAssetPlugin;
use bevy_inspector_egui::*;
use bevy_kira_audio::AudioSource;
use spritesheet_generator::sprite_sheet;
pub use dungeon_pack::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Tanks)
            .with_collection::<FontAssets>()
            .with_collection::<MusicAssets>()
            .with_collection::<PongAssets>()
            .with_collection::<DungeonPackAssets>()
            .init_resource::<DungeonPackAtlas>()
            .build(app);

        app.add_plugin(RonAssetPlugin::<sprite_sheet::SpriteSheet>::new(&[
            "gen.ron",
        ]))
        .add_plugin(InspectorPlugin::<DungeonPackAtlas>::new_insert_manually().open(true));
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

