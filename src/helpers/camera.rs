use bevy::{prelude::*, render::{camera::{Camera, DepthCalculation, OrthographicProjection}, render_graph::base}};

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let scale = scale + 0.1;
            transform.scale = Vec3::splat(scale);
        }

        if keyboard_input.pressed(KeyCode::X) {
            let scale = scale - 0.1;
            transform.scale = Vec3::splat(scale);
        }

        if transform.scale.x < 1.0 {
            transform.scale = Vec3::splat(1.0)
        }

        let move_dir = direction * 500.0 * time.delta_seconds();
        transform.translation += move_dir;

    }
}


// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn movement_relitive(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= transform.right() ;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += transform.right();
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += transform.up() ;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= transform.up() ;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let scale = scale + 0.1;
            transform.scale = Vec3::splat(scale);
        }

        if keyboard_input.pressed(KeyCode::X) {
            let scale = scale - 0.1;
            transform.scale = Vec3::splat(scale);
        }

        if transform.scale.x < 1.0 {
            transform.scale = Vec3::splat(1.0)
        }

        let move_dir = direction * 500.0 * time.delta_seconds();
        transform.translation += move_dir;
    }
}



#[allow(dead_code)]
pub fn new_2d_iso() -> OrthographicCameraBundle {
    // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
    // the camera's translation by far and use a right handed coordinate system
    let far = 2000.0;
    let offset = 500.0;
    let mut camera= OrthographicCameraBundle {
        camera: Camera {
            name: Some(base::camera::CAMERA_2D.to_string()),
            ..Default::default()
        },
        orthographic_projection: OrthographicProjection {
            far,
            near: 0.0,
            depth_calculation: DepthCalculation::ZDifference,
            ..Default::default()
        },
        visible_entities: Default::default(),
        transform: Transform {
            translation: Vec3::new(offset, offset * 0.75, offset),
            rotation: Quat::IDENTITY,
            ..Default::default()
        },

        global_transform: Default::default(),
    };

    camera.transform.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
    camera
}