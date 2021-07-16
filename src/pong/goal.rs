use crate::pong::ball;

use super::ball::Ball;
use super::events::GoalEvent;
use super::score::Score;
use super::{Player, Pong, PongData};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::WindowResized;

pub struct Goal;

impl Goal {
    const THICKNESS: f32 = 20.0;

    pub fn update_after_window_resize(
        &self,
        resize_event: &WindowResized,
        player: Player,
        size: &mut Vec2,
        translation: &mut Vec3,
    ) {
        let window_width = resize_event.width as f32;
        let window_height = resize_event.height as f32;
        *size = Vec2::new(Self::THICKNESS, window_height);

        use Player::*;
        let x_offset = (window_width - Self::THICKNESS) / 2.0;
        let x_position = match player {
            Left => x_offset,
            Right => -x_offset,
        };
        *translation = Vec3::new(x_position, 0.0, 0.0);
    }
}

pub fn spawn_goals(commands: &mut Commands, material: Handle<ColorMaterial>) {
    spawn_goal(commands, Player::Left, material.clone());
    spawn_goal(commands, Player::Right, material.clone());
}

fn spawn_goal(commands: &mut Commands, player: Player, material: Handle<ColorMaterial>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: material,
            ..Default::default()
        })
        .insert(Goal)
        .insert(player)
        .insert(Name::new(format!("{:?} Goal", player)))
        .insert(Pong);
}

pub fn goal_collision_system(
    ball_query: Query<(&Ball, &Transform, &Sprite)>,
    goal_query: Query<(&Transform, &Sprite, &Goal, &Player)>,
    mut ev_goal: EventWriter<GoalEvent>,
) {
    for (_ball, ball_transform, ball_sprite) in ball_query.iter() {
        for (goal_transform, goal_sprite, _goal, player) in goal_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.size,
                goal_transform.translation,
                goal_sprite.size,
            );

            if collision.is_some() {
                ev_goal.send(GoalEvent {
                    player: player.clone(),
                })
            }
        }
    }
}

pub fn goal_scored_event(
    mut commands: Commands,
    mut ev_goal: EventReader<GoalEvent>,
    query: Query<(Entity, &Ball)>,
    mut score: ResMut<Score>,
    data: Res<PongData>,
    window_desc: Res<WindowDescriptor>,
) {
    // Was a goal scored
    for goal in ev_goal.iter() {
        //add to the score
        use Player::*;
        match goal.player {
            Left => score.left += 1,
            Right => score.right += 1,
        }

        // Remove ball
        for (entity, _ball) in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // spawn new one
        ball::spawn_ball(
            &mut commands,
            data.primary_material.clone(),
            Vec2::new(window_desc.width, window_desc.height),
        );
    }
}