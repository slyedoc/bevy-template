use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::{GameState, helpers, loading::TextureAssets};

use super::TankCleanup;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Tanks), //.with_system(startup.system())
                                                   //.with_system(helpers::texture::set_texture_filters_to_nearest.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Tanks), //.with_system(update_map.system())
        );
    }
}

fn setup(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    // for handle in rpg_sprite_handles.handles.iter() {
    //     let texture = textures.get(handle).unwrap();
    //     texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    // }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_texture = texture_atlas.texture.clone();
    let vendor_handle = asset_server.get_handle("textures/rpg/chars/vendor/generic-rpg-vendor.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);
    // draw a sprite from the atlas
    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(150.0, 0.0, 0.0),
            scale: Vec3::splat(4.0),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(vendor_index as u32),
        texture_atlas: atlas_handle,
        ..Default::default()
    });
    // draw the atlas itself
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_atlas_texture.into()),
        transform: Transform::from_xyz(-300.0, 0.0, 0.0),
        ..Default::default()
    });
}
