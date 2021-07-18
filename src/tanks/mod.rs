mod map;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;
use crate::helpers;
use crate::{helpers::cleanup_system, GameState};

use map::MapPlugin;

#[derive(Inspectable, Debug)]
pub struct TanksData {
    clear_color: Color,

    #[inspectable(min = 0.0, max = 300.0, label = "Size")]
    size: f32,

    #[inspectable(min = 0.01, max = 20.0, label = "Line Thickness")]
    line_thickness: f32,

    board_material: Handle<ColorMaterial>,
    x_material: Handle<ColorMaterial>,
    o_material: Handle<ColorMaterial>,
    none_material: Handle<StandardMaterial>,
}

impl FromWorld for TanksData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");

        let mut standard_materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("ResMut<Assets<StandardMaterial>> not found.");

        TanksData {
            clear_color: Color::WHITE,
            board_material: materials.add(Color::BLACK.into()),
            o_material: materials.add(Color::BLUE.into()),
            x_material: materials.add(Color::RED.into()),
            none_material: standard_materials.add(StandardMaterial {
                base_color: Color::ANTIQUE_WHITE.into(),
                unlit: true,
                ..Default::default()
            }),
            size: 400.0,
            line_thickness: 10.0,
        }
    }
}

pub struct TanksPlugin;

impl Plugin for TanksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<TanksData>::new().open(false))
            .add_plugin(MapPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Tanks)
                .with_system(startup.system())
                .with_system(setup_spritesheet.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::Tanks)
                .with_system(update.system())
                .with_system(animate_sprite_system.system())
                .with_system(helpers::camera::movement.system())
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Tanks)
                    .with_system(cleanup_system::<TankCleanup>.system()),
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




use std::fs::File;

use spritesheet_generator::spritesheet::Spritesheet;

fn main() {
    // Open the file in read-only mode.
    let file = File::open("examples/resources/example.json").unwrap();

    // Read the JSON contents of the file as an instance of `Spritesheet`.
    let sprites: Spritesheet = serde_json::from_reader(file).unwrap();
    println!("Json parsed {:?}", sprites);

    // Read one specific frame.
    let frame = sprites.frames.get("battlehammer").unwrap().screen.clone();
    println!("Frame parsed {:?}", frame);
}


fn setup_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true));
}




#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct TankCleanup;

fn startup(
    mut commands: Commands,
    data: Res<TanksData>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = data.clear_color;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


fn update(
    mut clear_color: ResMut<ClearColor>,
    data: Res<TanksData>,
) {
    if data.is_changed() {
        clear_color.0 = data.clear_color;
    }
}


