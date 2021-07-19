use bevy::{prelude::*, render::texture::FilterMode, sprite};

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Texture>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.sampler.min_filter = FilterMode::Nearest;
                }
            }
            _ => (),
        }
    }
}

 /// Generate a `TextureAtlas` by splitting a texture into a grid where each
    /// cell of the grid of `tile_size` is one of the textures in the atlas and is separated by
    /// some `padding` in the texture. The padding is assumed to be only between tiles
    /// and not at the borders of the texture.
    pub fn from_sprite_gen(
        texture: Handle<Texture>,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        padding: Vec2,
    ) -> TextureAtlas {
        let mut sprites = Vec::new();
        let mut x_padding = 0.0;
        let mut y_padding = 0.0;

        for y in 0..rows {
            if y > 0 {
                y_padding = padding.y;
            }
            for x in 0..columns {
                if x > 0 {
                    x_padding = padding.x;
                }

                let rect_min = Vec2::new(
                    (tile_size.x + x_padding) * x as f32,
                    (tile_size.y + y_padding) * y as f32,
                );

                sprites.push(sprite::Rect {
                    min: rect_min,
                    max: Vec2::new(rect_min.x + tile_size.x, rect_min.y + tile_size.y),
                })
            }
        }

        TextureAtlas {
            size: Vec2::new(
                ((tile_size.x + x_padding) * columns as f32) - x_padding,
                ((tile_size.y + y_padding) * rows as f32) - y_padding,
            ),
            textures: sprites,
            texture,
            texture_handles: None,
        }
    }