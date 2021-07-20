use bevy::prelude::*;
use super::TankCleanup;
use crate::{GameState, loading::DungeonPackAtlas};
use rand::{self, Rng};

pub struct MapPlugin {
    state: GameState,

}
impl MapPlugin {
    pub fn new(state: GameState) -> Self {
        MapPlugin { state: state }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(self.state.clone()).with_system(startup.system()), //.with_system(helpers::texture::set_texture_filters_to_nearest.system())
        );
    }
}

fn startup(
    mut commands: Commands,
    pack: Res<DungeonPackAtlas>,
) {

    let size  = Vec2::new (256.0, 148.0);
    let mut rng = rand::thread_rng();

    // spawn floor
    let mut count = -0.0;
    for x in -1..1 {
        for y in -1..1 {
            // each isometric sprite takes half the space in a grid, so we are just going to create a second
            // SpriteSheetBundle with and offset, for draw order we will use a z offset, going to need more I think
            let iso_y = 0.95 * y as f32;
            let random_floor_piece =  pack.floors[rng.gen_range( 0..pack.floors.len() ) ];
            println!("{}", pack.frames[&random_floor_piece]);
            commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3::splat(1.0),
                    translation: Vec3::new(size.x * x as f32 + size.x * 0.5, size.y * iso_y as f32 + size.y * 0.5, count as f32),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(random_floor_piece ),
                texture_atlas: pack.atlas.clone(),
                ..Default::default()
            })
            .insert(TankCleanup);



            count += 5.0;
        }
    }

}

