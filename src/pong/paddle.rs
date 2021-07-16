use super::wall::Wall;
use super::{Collider, Player, Pong, PongAction};
use bevy::prelude::*;
use bevy::window::{WindowDescriptor, WindowResized};
use bevy_input_actionmap::*;

#[derive(Default)]
pub struct Paddle {
    speed: f32,
}

impl Paddle {
    const WIDTH: f32 = 20.0;
    const MARGIN: f32 = 50.0;

    pub fn update_after_window_resize(
        &mut self,
        resize_event: &WindowResized,
        player: Player,
        size: &mut Vec2,
        translation: &mut Vec3,
    ) {
        let window_height = resize_event.height as f32;
        let window_width = resize_event.width as f32;
        self.speed = (window_height as f32) / 3.0;

        *size = Vec2::new(Paddle::WIDTH, 0.2 * window_height);

        use Player::*;
        let x_translation = match player {
            Left => Paddle::MARGIN - (window_width / 2.0),
            Right => (window_width / 2.0) - Paddle::MARGIN,
        };

        *translation = Vec3::new(x_translation, 0.0, 0.0);
    }
}

pub fn spawn_paddles(commands: &mut Commands, material: Handle<ColorMaterial>) {
    spawn_paddle(commands, Player::Left, material.clone());
    spawn_paddle(commands, Player::Right, material.clone());
}

fn spawn_paddle(commands: &mut Commands, player: Player, material: Handle<ColorMaterial>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: material,
            ..Default::default()
        })
        .insert(Paddle::default())
        .insert(player)
        .insert(Collider)
        .insert(Name::new(format!("{:?} Paddle", player)))
        .insert(Pong);
}


pub fn paddle_movement_system(
    time: Res<Time>,
    input_map: Res<InputMap<PongAction>>,
    mut query: Query<(&Paddle, &Player, &mut Transform)>,
    window_desc: Res<WindowDescriptor>,
) {
    let time_delta = time.delta_seconds();

    for (paddle, player, mut transform) in query.iter_mut() {
        let (up_action, down_action) = player.movement_actions();

        if input_map.active(up_action) {
            transform.translation += time_delta * Vec3::new(0.0, paddle.speed, 0.0);
        }

        if input_map.active(down_action) {
            transform.translation += time_delta * Vec3::new(0.0, -paddle.speed, 0.0);
        }

        // Clamp paddles so they dont go off the screen
        let range = window_desc.height * 0.4 - Wall::THICKNESS;
        transform.translation.y = transform.translation.y.clamp(-range, range);
    }
}


