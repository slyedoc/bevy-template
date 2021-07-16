use super::{Collider, Pong};
use bevy::core::Name;
use bevy::ecs::system::Commands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Handle;
use bevy::sprite::entity::SpriteBundle;
use bevy::sprite::ColorMaterial;
use bevy::window::WindowResized;

pub enum Wall {
    Top,
    Bottom,
}

impl Wall {
    pub const THICKNESS: f32 = 20.0;

    pub fn update_after_window_resize(
        &self,
        resize_event: &WindowResized,
        size: &mut Vec2,
        translation: &mut Vec3,
    ) {
        let window_width = resize_event.width as f32;
        let window_height = resize_event.height as f32;
        *size = Vec2::new(window_width, Self::THICKNESS);

        use Wall::*;
        let y_offset = (window_height - Self::THICKNESS) / 2.0;
        let y_position = match self {
            Top => y_offset,
            Bottom => -y_offset,
        };
        *translation = Vec3::new(0.0, y_position, 0.0);
    }
}

pub fn spawn_walls(commands: &mut Commands, material: Handle<ColorMaterial>) {
    spawn_wall(commands, Wall::Top, material.clone());
    spawn_wall(commands, Wall::Bottom, material.clone());
}

fn spawn_wall(commands: &mut Commands, wall: Wall, material: Handle<ColorMaterial>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: material,
            ..Default::default()
        })
        .insert(wall)
        .insert(Collider)
        .insert(Name::new("Wall"))
        .insert(Pong);
}
