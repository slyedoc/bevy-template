use bevy::{prelude::*, sprite::Rect, sprite::TextureAtlasBuilder};

use crate::{GameState, loading::{TankAssets, TextureAssets}};
use spritesheet_generator::sprite_sheet::{self, SpriteSheet};
use super::TankCleanup;

pub struct MapPlugin {
    state: GameState,
}
impl MapPlugin {
    pub fn new( state: GameState ) -> Self {
        MapPlugin {
            state: state,
        }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(self.state.clone())
            .with_system(startup.system())
            //.with_system(helpers::texture::set_texture_filters_to_nearest.system())
        )
        .add_system_set(
            SystemSet::on_update(self.state.clone())
            .with_system(animate_sprite_system.system())
        );
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn startup(
    mut commands: Commands,
    tank_assets: Res<TankAssets>,
    assets: Res<Assets<sprite_sheet::SpriteSheet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle = tank_assets.stone_floor.clone();
    // everything already should be loaded
    let sheet  = assets.get(&tank_assets.stone_floor_sheet).unwrap();
    let texture_atlas = from_gen(texture_handle, sheet);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(TankCleanup);
}

 pub fn from_gen(
        texture: Handle<Texture>,
        sprite_sheet: &SpriteSheet,
    ) -> TextureAtlas {
        let mut sprites = Vec::new();
        for f in sprite_sheet.frames.iter() {

            sprites.push(Rect {
                min: Vec2::new(
                    f.x as f32,
                    f.y as f32,
                ),
                max: Vec2::new( (f.x + f.w) as f32, (f.y + f.h) as f32),
            })
        }

        TextureAtlas {
            size: Vec2::new( 256.0, 148.0),
            textures: sprites,
            texture,
            texture_handles: None,
        }
    }

    // let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    // let texture_atlas_texture = texture_atlas.texture.clone();
    // let vendor_handle = asset_server.get_handle("textures/rpg/chars/vendor/generic-rpg-vendor.png");
    // let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    // let atlas_handle = texture_atlases.add(texture_atlas);
    // // draw a sprite from the atlas
    // commands.spawn_bundle(SpriteSheetBundle {
    //     transform: Transform {
    //         translation: Vec3::new(150.0, 0.0, 0.0),
    //         scale: Vec3::splat(4.0),
    //         ..Default::default()
    //     },
    //     sprite: TextureAtlasSprite::new(vendor_index as u32),
    //     texture_atlas: atlas_handle,
    //     ..Default::default()
    // })
    // .insert(TankCleanup);
    // // draw the atlas itself
    // commands.spawn_bundle(SpriteBundle {
    //     material: materials.add(texture_atlas_texture.into()),
    //     transform: Transform::from_xyz(-300.0, 0.0, 0.0),
    //     ..Default::default()
    // })
    // .insert(TankCleanup);


