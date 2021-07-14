// From https://github.com/FSMaxB/bevy-pong-clone
mod ball;
mod goal;
mod paddle;
mod score;
mod wall;

use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;

use crate::{helpers::cleanup_system, GameState};

use self::ball::{ball_collision_system, ball_movement_system, Ball};
use self::goal::{goal_collision_system, Goal};
use self::paddle::{paddle_movement_system, Paddle};
use self::score::Score;
use self::wall::Wall;

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
        app.insert_resource(score::Score::default())
            .add_plugin(InspectorPlugin::<PongData>::new().open(false))
            .add_system_set(SystemSet::on_enter(GameState::Pong).with_system(setup.system())
            .with_system(window_resize_listener.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Pong)
                    .with_system(ball_movement_system.system())
                    .with_system(paddle_movement_system.system())
                    .with_system(window_resize_listener.system())
                    .with_system(ball_collision_system.system())
                    .with_system(goal_collision_system.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Pong).with_system(cleanup_system::<Pong>.system())
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Left,
    Right,
}

impl Player {
    fn movement_keys(&self) -> (KeyCode, KeyCode) {
        match self {
            Player::Left => (KeyCode::W, KeyCode::S),
            Player::Right => (KeyCode::Up, KeyCode::Down),
        }
    }
}

struct Pong;

pub struct Collider;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    data: Res<PongData>,
    mut clear_color: ResMut<ClearColor>,
    windows: Res<Windows>,
    window_desc: Res<WindowDescriptor>,
    mut window_resize: EventWriter<WindowResized>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Pong);
    commands
        .spawn()
        .insert_bundle(UiCameraBundle::default())
        .insert(Pong);
    ball::spawn_ball(&mut commands);
    paddle::spawn_paddles(&mut commands);
    wall::spawn_walls(&mut commands);
    goal::spawn_goals(&mut commands);
    score::spawn_score_board(&mut commands, &asset_server);

    clear_color.0 = data.background;

    let window = windows.get_primary();
    window_resize.send(WindowResized {
        id: window.unwrap().id(),
        width: window_desc.width,
        height: window_desc.height,
    });
}

fn window_resize_listener(
    mut resize_reader: EventReader<WindowResized>,
    mut query_set: QuerySet<(
        Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
        Query<(&mut Sprite, &mut Transform, &Wall)>,
        Query<(&mut Sprite, &mut Transform, &Goal, &Player)>,
        Query<(&mut Sprite, &mut Transform, &mut Ball)>,
    )>,
    score: Res<Score>,
) {
    if let Some(resize_event) = resize_reader.iter().last() {
        println!("Score: {}", *score);
        let width = resize_event.width;
        let height = resize_event.height;
        println!("Window resized to {}x{}", width, height);

        let paddles = query_set.q0_mut();
        for (mut sprite, mut transform, mut paddle, player) in paddles.iter_mut() {
            paddle.update_after_window_resize(
                resize_event,
                *player,
                &mut sprite.size,
                &mut transform.translation,
            );
        }

        let walls = query_set.q1_mut();
        for (mut sprite, mut transform, wall) in walls.iter_mut() {
            wall.update_after_window_resize(
                resize_event,
                &mut sprite.size,
                &mut transform.translation,
            );
        }

        let goals = query_set.q2_mut();
        for (mut sprite, mut transform, goal, player) in goals.iter_mut() {
            goal.update_after_window_resize(
                resize_event,
                *player,
                &mut sprite.size,
                &mut transform.translation,
            );
        }

        let ball = query_set.q3_mut();
        for (mut sprite, mut transform, mut ball) in ball.iter_mut() {
            ball.update_after_window_resize(
                resize_event,
                &mut sprite.size,
                &mut transform.translation,
            );
        }
    }
}
