use bevy::{prelude::*, sprite};
use spritesheet_generator::sprite_sheet::SpriteSheet;


// Used to import our own custom TextureAtlas from sprite sheet generator
pub fn from_sprite_sheet(texture: Handle<Texture>, sprite_sheet: &SpriteSheet) -> TextureAtlas {
    let mut sprites = Vec::new();
    for f in sprite_sheet.frames.iter() {
        let rec = sprite::Rect {
            min: Vec2::new(f.position.x as f32, f.position.y as f32),
            max: Vec2::new(
                (f.position.x + f.position.w) as f32,
                (f.position.y + f.position.h) as f32,
            ),
        };
        sprites.push(rec)
    }

    TextureAtlas {
        size: Vec2::new(sprite_sheet.width as f32, sprite_sheet.height as f32),
        textures: sprites,
        texture,
        texture_handles: None,
    }
}
