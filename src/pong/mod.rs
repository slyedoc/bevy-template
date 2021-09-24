// From https://github.com/FSMaxB/bevy-pong-clone
mod audio;
mod ball;
mod events;
mod goal;
mod paddle;
mod score;
mod wall;

use crate::GameState;
use crate::audio::AudioState;
use crate::helpers::*;
use audio::*;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_input_actionmap::*;
use bevy_inspector_egui::widgets::ResourceInspector;
use bevy_inspector_egui::*;
use std::fmt;

use self::ball::*;
use self::events::*;
use self::goal::*;
use self::paddle::*;
use self::score::*;
use self::wall::*;

#[derive(Inspectable, Debug)]
pub struct PongData {
    #[inspectable(label = "Background Color")]
    background: Color,

    primary_material: Handle<ColorMaterial>,

    score: ResourceInspector<Score>,

    audio: AudioState,

}



impl FromWorld for PongData {
    fn from_world(world: &mut World) -> Self {

        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");

        PongData {
            background: Color::BLACK,
            primary_material: materials.add(Color::WHITE.into()),
            score: ResourceInspector::<Score>::default(),
            audio: AudioState::default(),
        }
    }
}

pub struct PongPlugin;
impl Plugin for PongPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(score::Score::default())
            .add_event::<GoalEvent>()
            .add_event::<BallBounceEvent>()
            .add_plugin(InspectorPlugin::<PongData>::new().open(false))
            .add_plugin(ActionPlugin::<PongAction>::default())
            .add_system_set(
                SystemSet::on_enter(GameState::Pong)
                    .with_system(setup.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::Pong)
                    .with_system(handle_audio_events.system())
                    .with_system(update_clear_color_system.system())
                    .with_system(paddle_movement_system.system().label("input"))
                    .with_system(ball_movement_system.system().after("input"))
                    .with_system(window_resize_listener.system())
                    .with_system(ball_collision_system.system())
                    .with_system(goal_collision_system.system().label("goal"))
                    .with_system(goal_scored_event.system().after("goal"))
                    .with_system(update_score_board.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Pong)
                    .with_system(cleanup_system::<Pong>.system())
                    .with_system(cleanup_actions_system::<PongAction>.system())
            );
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PongAction {
    Player1Up,
    Player1Down,
    Player2Up,
    Player2Down,
}

impl fmt::Display for PongAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PongAction::Player1Up => write!(f, "Player 1 - Up"),
            PongAction::Player1Down => write!(f, "Player 1 - Down"),
            PongAction::Player2Up => write!(f, "Player 2 - Up"),
            PongAction::Player2Down => write!(f, "Player 2 - Down"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Left,
    Right,
}

impl Player {
    fn movement_actions(&self) -> (PongAction, PongAction) {
        match self {
            Player::Left => (PongAction::Player1Up, PongAction::Player1Down),
            Player::Right => (PongAction::Player2Up, PongAction::Player2Down),
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
    mut input: ResMut<InputMap<PongAction>>
) {
    // TODO: Editor really should remove this cameras, but works, leaving for now
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Pong);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Pong);

    ball::spawn_ball(
        &mut commands,
        data.primary_material.clone(),
        Vec2::new(window_desc.width, window_desc.height),
    );
    paddle::spawn_paddles(&mut commands, data.primary_material.clone());
    wall::spawn_walls(&mut commands, data.primary_material.clone());
    goal::spawn_goals(&mut commands, data.primary_material.clone());
    score::spawn_score_board(&mut commands, &asset_server);

    // Update our background
    clear_color.0 = data.background;

    // TODO: This is a hack to reuse the logic in window_resize_listener, should pass size into
    // Already did this for ball, needed it for respawn
    let window = windows.get_primary();
    window_resize.send(WindowResized {
        id: window.unwrap().id(),
        width: window_desc.width,
        height: window_desc.height,
    });


    input
        .bind(PongAction::Player1Up, KeyCode::W)
        .bind(PongAction::Player1Down, KeyCode::S)
        .bind(PongAction::Player2Up, KeyCode::Up)
        .bind(PongAction::Player2Down, KeyCode::Down);

}

// TODO: This entire system is only needed because I want really time feedback in inspector
fn update_clear_color_system(data: Res<PongData>, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = data.background;
}

fn window_resize_listener(
    mut resize_reader: EventReader<WindowResized>,
    mut query_set: QuerySet<(
        Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
        Query<(&mut Sprite, &mut Transform, &Wall)>,
        Query<(&mut Sprite, &mut Transform, &Goal, &Player)>,
        Query<(&mut Sprite, &mut Transform, &mut Ball)>,
    )>,
) {
    if let Some(resize_event) = resize_reader.iter().last() {
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
