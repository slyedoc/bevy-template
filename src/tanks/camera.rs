use std::fmt;

use bevy::{prelude::*, render::{camera::*, render_graph::base}};
use bevy_input_actionmap::{ActionPlugin, InputMap};
use bevy_mod_picking::PickingCameraBundle;
use bevy_prototype_debug_lines::DebugLines;

use crate::{GameState, editor::EditorState, helpers::cleanup_system};

struct CameraComponent {
    movement_speed: f32,
}
struct CameraCleanup;

pub struct CameraPlugin {
    state: GameState,
}

impl CameraPlugin {
    pub fn new(state: GameState) -> Self {
        CameraPlugin { state: state }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ActionPlugin::<CameraActions>::default())
            .add_system_set(
                SystemSet::on_enter(self.state.clone())
                    .with_system(startup.system())
                    .with_system(setup_camera_actions.system()),
            )
            .add_system_set(
                SystemSet::on_update(self.state.clone()).with_system(run_actions.system()),
            )
            .add_system_set(
                SystemSet::on_exit(self.state.clone())
                    .with_system(clear_camera_actions.system())
                    .with_system(cleanup_system::<CameraCleanup>.system()),
            )
            .add_system_set(
                SystemSet::on_update(EditorState::Playing).with_system(draw_gizmo.system()),
            );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum CameraActions {
    Up,
    Down,
    Left,
    Right,
    In,
    Out,
}

impl fmt::Display for CameraActions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CameraActions::Up => write!(f, "Move Camera Up"),
            CameraActions::Down => write!(f, "Move Camera Down"),
            CameraActions::Left => write!(f, "Move Camera Left"),
            CameraActions::Right => write!(f, "Move Camera Right"),
            CameraActions::In => write!(f, "Zoom Camera In"),
            CameraActions::Out => write!(f, "Zoom Camera Out"),
        }
    }
}

fn startup(mut commands: Commands) {
    let offset = 3500.0;
    let mut camera = PerspectiveCameraBundle {
    //let mut camera = OrthographicCameraBundle {
        camera: Camera {
            name: Some(base::camera::CAMERA_2D.to_string()),
            ..Default::default()
        },
        //orthographic_projection: OrthographicProjection {
        perspective_projection: PerspectiveProjection {
            far: 4000.0,
            ..Default::default()
        },
        visible_entities: Default::default(),
        transform: Transform {
            translation: Vec3::new(0.0, offset, 0.0),
            rotation: Quat::IDENTITY,
            ..Default::default()
        },

        global_transform: Default::default(),
    };

    // // easyer to use lookat that try and set the quat
    camera.transform.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z);

    commands.spawn_bundle(camera).insert(CameraComponent {
        movement_speed: 500.0,
    })
    .insert_bundle(PickingCameraBundle::default())
    .insert(CameraCleanup);

    commands
    .spawn_bundle(UiCameraBundle::default())
    .insert(CameraCleanup);

}

fn setup_camera_actions(mut input: ResMut<InputMap<CameraActions>>) {
    input
        .bind(CameraActions::Up, KeyCode::W)
        .bind(CameraActions::Down, KeyCode::S)
        .bind(CameraActions::Left, KeyCode::A)
        .bind(CameraActions::Right, KeyCode::D)
        .bind(CameraActions::Up, KeyCode::Up)
        .bind(CameraActions::Down, KeyCode::Down)
        .bind(CameraActions::Left, KeyCode::Left)
        .bind(CameraActions::Right, KeyCode::Right)
        .bind(CameraActions::In, KeyCode::Z)
        .bind(CameraActions::Out, KeyCode::X);
}

fn clear_camera_actions(mut input: ResMut<InputMap<CameraActions>>) {
    input.clear();
}

fn run_actions(
    time: Res<Time>,
    input: Res<InputMap<CameraActions>>,
    mut query: Query<(&mut Transform, &CameraComponent)>,
) {
    for (mut transform, camera) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        if input.active(CameraActions::Left) {
            direction -= transform.right();
        }

        if input.active(CameraActions::Right) {
            direction += transform.right();
        }

        if input.active(CameraActions::Up) {
            direction += transform.up();
        }

        if input.active(CameraActions::Down) {
            direction -= transform.up();
        }

        if input.active(CameraActions::In) {
            direction -= transform.forward();
        }

        if input.active(CameraActions::Out) {
            direction += transform.forward();
        }

        let move_dir = direction * camera.movement_speed * time.delta_seconds();
        //println!("{:?}", move_dir);
        transform.translation += move_dir;
    }
}

fn draw_gizmo(
    mut lines: ResMut<DebugLines>,
    query: Query<(&Transform, &PerspectiveProjection), With<CameraComponent>>,
) {

    for (transform, proj) in query.iter() {
        let crosshair_size = 10.0;
        lines.line(
            transform.translation - transform.right() * crosshair_size,
            transform.translation + transform.right() * crosshair_size,
            0.0,
        );

        lines.line(
            transform.translation - transform.up() * crosshair_size,
            transform.translation + transform.up() * crosshair_size,
            0.0,
        );

        lines.line(
            transform.translation,
            transform.translation + transform.forward() * proj.far,
            0.0,
        );
    }
}
