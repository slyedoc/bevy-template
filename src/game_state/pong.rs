use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;
use bevy_inspector_egui::{Inspectable};

use crate::{GameState, helpers::cleanup_system};

#[derive(Inspectable, Debug)]
pub struct PongData {

    #[inspectable(label = "Background Color")]
    background: Color,

    primary_material: Handle<ColorMaterial>,
}

impl FromWorld for PongData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");

        PongData {
            background: Color::BLACK,
            primary_material: materials.add(Color::WHITE.into()),
        }
    }
}

pub struct PongPlugin;
impl Plugin for PongPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(InspectorPlugin::<PongData>::new().open(false))
            .add_system_set(SystemSet::on_enter(GameState::Pong).with_system(setup.system()))
            .add_system_set(SystemSet::on_exit(GameState::Pong).with_system(cleanup_system::<PongAsset>.system()));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

enum PongAsset {
    Left,
    Right,
    Ball,
}

fn setup(mut commands: Commands, data: Res<PongData>, mut clear_color: ResMut<ClearColor>) {
    let offset = 300.0;
    clear_color.0 = data.background;

    // Create left Paddle
    commands
        .spawn_bundle(create_paddle_sprite(-offset, data.primary_material.clone()))
        .insert(Name::new("Paddle Left"))
        .insert(PongAsset::Left);

    // Create right Paddle
    commands
        .spawn_bundle(create_paddle_sprite(offset, data.primary_material.clone()))
        .insert(Name::new("Paddle Right"))
        .insert(PongAsset::Right);

    // Create Ball
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(10.0, 10.0),
                ..Default::default()
            },
            material: data.primary_material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PongAsset::Ball);

    // Spawn Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn create_paddle_sprite(offset_y: f32, material: Handle<ColorMaterial>) -> SpriteBundle {
    let paddle_size = Vec2::new(20.0, 100.0);
    SpriteBundle {
        sprite: Sprite {
            size: paddle_size,
            ..Default::default()
        },
        material: material,
        transform: Transform {
            translation: Vec3::new(offset_y, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    }
}
