use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::PerspectiveProjection,
};
use bevy_input_actionmap::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::PickingCameraBundle;

use crate::helpers::{cleanup_actions_system, cleanup_system};
use std::fmt;
use super::EditorState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ActionPlugin::<EditorCameraAction>::default())
        .add_system_set(
            SystemSet::on_enter(EditorState::Loading)
                .with_system(spawn_cameras.system()),
        )
        .add_system_set(
            SystemSet::on_exit(EditorState::Playing)
                .with_system(cleanup_system::<EditorCamera>.system())
                .with_system(cleanup_actions_system::<EditorCameraAction>.system())
        )
        .add_system(pan_orbit_camera.system());
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum EditorCameraAction {
    Orbit,
    Pan,
}

impl fmt::Display for EditorCameraAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorCameraAction::Orbit => write!(f, "Orbit Camera"),
            EditorCameraAction::Pan => write!(f, "Pan Camera"),
        }
    }
}

/// Marker component for editor game camera
#[derive(Inspectable, Debug)]
pub enum EditorCamera {
    UI,
    Perspective,
}

/// Spawn a camera like this
#[allow(dead_code)]
pub fn spawn_cameras(mut commands: Commands, mut input_map: ResMut<InputMap<EditorCameraAction>>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(EditorCamera::UI);


    let location = Vec3::new(100.0, 100.0, 600.0);
    let radius = location.length();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(location).looking_at(Vec3::ZERO, Vec3::Y),
            perspective_projection: PerspectiveProjection {
                far: f32::MAX,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default())
        .insert(EditorCamera::Perspective)
        .insert(Name::new("EditorCamera"));

        input_map
            .bind(EditorCameraAction::Orbit, MouseButton::Right)
            .bind(EditorCameraAction::Pan, MouseButton::Middle);

}

/// Tags an entity as capable of panning and orbiting.
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

// This is from the bevy cheatbook
/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_map: Res<InputMap<EditorCameraAction>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
) {
    // change input mapping for orbit and panning here


    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_map.active(EditorCameraAction::Orbit) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_map.active(EditorCameraAction::Pan){
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_map.just_inactive(EditorCameraAction::Orbit) || input_map.just_active(EditorCameraAction::Orbit) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
