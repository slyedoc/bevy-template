use super::events::BallBounceEvent;
use super::{Collider, Pong};
use bevy::prelude::*;

use bevy::ecs::system::{Commands, Query, Res};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Handle;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use bevy::sprite::entity::SpriteBundle;
use bevy::sprite::{ColorMaterial, Sprite};
use bevy::transform::components::Transform;
use bevy::window::WindowResized;
use rand::Rng;

pub struct Ball {
    speed: f32,
    direction: Vec2,
}

impl Ball {
    // Builder function to set speed
    pub fn speed(self, window_size: Vec2) -> Self {
        Ball {
            speed: window_size.y / 1.5,
            ..self
        }
    }

    //gets current velocity
    pub fn velocity(&self) -> Vec2 {
        self.speed * self.direction.normalize()
    }

    pub fn update_after_window_resize(
        &mut self,
        resize_event: &WindowResized,
        size: &mut Vec2,
        translation: &mut Vec3,
    ) {
        let window_height = resize_event.height as f32;
        self.speed = window_height / 1.5;
        *size = self.get_size(Vec2::new(resize_event.width, resize_event.height));

        *translation = Vec3::default();
    }

    pub fn get_size(&self, window_size: Vec2) -> Vec2 {
        let ball_width = 0.05 * window_size.y;
        Vec2::new(ball_width, ball_width)
    }
}

impl Default for Ball {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_bool(0.5);
        let angle = Vec2::new(
            if direction {
                rng.gen_range(-1.0..=-0.5)
            } else {
                rng.gen_range(0.5..=1.0)
            },
            rng.gen_range(-1.0..=1.0),
        )
        .normalize();

        Self {
            speed: Default::default(),
            direction: angle,
        }
    }
}

pub fn spawn_ball(commands: &mut Commands, material: Handle<ColorMaterial>, window_size: Vec2) {
    let ball = Ball::default().speed(window_size);

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: ball.get_size(window_size),
                ..Default::default()
            },
            material: material,
            ..Default::default()
        })
        .insert(ball)
        .insert(Name::new("Ball"))
        .insert(Pong);
}

pub fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    let time_delta = time.delta_seconds();
    for (ball, mut transform) in query.iter_mut() {
        transform.translation += time_delta * ball.velocity().extend(0.0);
    }
}

pub fn ball_collision_system(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(&Collider, &Transform, &Sprite)>,
    mut ev_ball_bounce: EventWriter<BallBounceEvent>,
) {
    for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
        for (_collider, collider_transform, collider_sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.size,
                collider_transform.translation,
                collider_sprite.size,
            );

            let collision = match collision {
                Some(collision) => collision,
                None => continue,
            };

            // Play Sound
            ev_ball_bounce.send(BallBounceEvent);

            use Collision::*;
            let (reflect_x, reflect_y) = match collision {
                Left => (ball.direction.x > 0.0, false),
                Right => (ball.direction.x < 0.0, false),
                Top => (false, ball.direction.y < 0.0),
                Bottom => (false, ball.direction.y > 0.0),
            };

            if reflect_x {
                ball.direction.x = -ball.direction.x;
            }

            if reflect_y {
                ball.direction.y = -ball.direction.y;
            }
        }
    }
}

