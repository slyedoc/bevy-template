use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::GameState;

use super::TankCleanup;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
                SystemSet::on_enter(GameState::Tanks)
                .with_system(startup.system())
                //.add_system(helpers::texture::set_texture_filters_to_nearest.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::Tanks)
                .with_system(update_map.system())
            );
    }
}

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let texture_handle = asset_server.load("tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn()
    .insert(TankCleanup)
    .insert(Name::new("Map"))
    .id();

    let mut map = Map::new(0u16, map_entity);

    let (layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            UVec2::new(2, 2).into(),
            UVec2::new(8, 8).into(),
            Vec2::new(20.0, 20.0),
            Vec2::new(96.0, 256.0),
        ),
        0u16,
        0u16,
        None,
    );

    map_query.build_layer(&mut commands, layer_builder, material_handle);

    commands.entity(layer_entity).insert(LastUpdate::default());

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}

fn build_map(map_query: &mut MapQuery, commands: &mut Commands) {
    let mut random = thread_rng();

    for _ in 0..100 {
        let position = UVec2::new(random.gen_range(0..16), random.gen_range(0..16));
        // Ignore errors for demo sake.
        let _ = map_query.set_tile(
            commands,
            position,
            Tile {
                texture_index: 0,
                ..Default::default()
            },
            0u16,
            0u16,
        );
        map_query.notify_chunk_for_tile(position, 0u16, 0u16);
    }
}

#[derive(Default)]
pub struct LastUpdate {
    value: f64,
}


pub fn update_map(
    time: ResMut<Time>,
    mut commands: Commands,
    mut query: Query<&mut LastUpdate>,
    mut map_query: MapQuery,
) {
    let current_time = time.seconds_since_startup();
    for mut last_update in query.iter_mut() {
        if (current_time - last_update.value) > 1.0 {
            map_query.despawn_layer_tiles(&mut commands, 0u16, 0u16);
            build_map(&mut map_query, &mut commands);
            last_update.value = current_time;
        }
    }
}
