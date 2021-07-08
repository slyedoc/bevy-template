pub mod data;
pub use data::*;


use super::{run_if_editor, EditorState};
use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;
use std::f32::consts::PI;
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<GridSpawnEvent>()
            .add_event::<GridClearEvent>()
            .add_plugin(InspectorPlugin::<GridData>::new().open(false))
            .add_system_set(
                SystemSet::on_enter(EditorState::Loading).with_system(spawn_grids.system()),

            )
            .add_system_set(
                SystemSet::on_exit(EditorState::Playing).with_system(clear_grid.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_editor.system())
                    .with_system(clear_grid_button.system()),
            );
    }
}

#[derive(Default)]
pub struct GridSpawnEvent;

#[derive(Default)]
pub struct GridResetEventButton;

#[derive(Default)]
pub struct GridClearEvent;

#[derive(Bundle, Clone, Debug)]
pub struct GridBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for GridBundle {
    fn default() -> Self {
        GridBundle {
            transform: Transform::default(),
            global_transform: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

struct Grid;
struct GridLine;

#[derive(Copy, Clone, Debug)]
enum GridType {
    X,
    Y,
    Z,
}

fn clear_grid(
    mut commands: Commands,
    q: Query<Entity, With<Grid>>,
) {


    println!("clear grid");
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }

}

fn clear_grid_button(
    mut commands: Commands,
    mut ev_clear: EventReader<GridResetEventButton>,
    data: Res<GridData>,
    q: Query<Entity, With<Grid>>,
) {
    for _ in ev_clear.iter() {
        for e in q.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}


pub fn spawn_grids(
    mut commands: Commands,
    grid: Res<GridData>,
) {
    println!("spawn");

        if grid.show_x_grid {
            build_grid(&mut commands, &grid, GridType::X);
        }

        if grid.show_y_grid {
            build_grid(&mut commands, &grid, GridType::Y);
        }

        if grid.show_z_grid {
            build_grid(&mut commands, &grid, GridType::Z);
        }
}

fn build_grid(commands: &mut Commands, grid: &Res<GridData>, grid_type: GridType) -> Entity {
    // TODO: I am not proud of this, its but its close to working and good enough for the moment
    // far to much code for what I am trying to and after all of that I still don't have the
    // rotations quite right so the lines dont match up at the corner
    let (transform, size) = match grid_type {
        GridType::X => (
            Transform {
                translation: if grid.grid_center {
                    Vec3::ZERO
                } else {
                    Vec3::new(grid.size.0 as f32 * -0.5, 0.0, 0.0)
                },
                rotation: Quat::from_rotation_y(PI / 2.0),
                ..Default::default()
            },
            (grid.size.1 as f32, grid.size.2 as f32),
        ),
        GridType::Y => (
            Transform {
                translation: if grid.grid_center {
                    Vec3::ZERO
                } else {
                    Vec3::new(0.0, grid.size.1 as f32 * -0.5, 0.0)
                },
                rotation: Quat::from_rotation_x(PI / 2.0),
                ..Default::default()
            },
            (grid.size.2 as f32, grid.size.0 as f32),
        ),
        GridType::Z => (
            Transform {
                // blue
                translation: if grid.grid_center {
                    Vec3::ZERO
                } else {
                    Vec3::new(0.0, 0.0, grid.size.2 as f32 * -0.5)
                },
                rotation: Quat::from_rotation_z(PI / 2.0),
                ..Default::default()
            },
            (grid.size.0 as f32, grid.size.1 as f32),
        ),
    };

    let parent = commands
        .spawn_bundle(GridBundle {
            transform: transform,
            ..Default::default()
        })
        .insert(grid_type) // so we get cleaned up
        .insert(Grid)
        .insert(Name::new(format!("Grid {:?}", grid_type)))
        .id();

    let mut lines: Vec<Entity> = vec![];

    let row_count = (size.0 / grid.cell_size).round() as u32;
    for row in 0..=row_count {
        let line = commands
            .spawn_bundle(build_gridline(
                Orientation::Horizontal,
                row,
                row == 0 || row == row_count || row % 5 == 0,
                &grid,
                grid_type,
                size,
            ))
            .insert(grid_type)
            .insert(GridLine)
            .id();
        lines.push(line);
    }

    let column_count = (size.1 / grid.cell_size).round() as u32;

    for column in 0..=column_count {
        let line = commands
            .spawn_bundle(build_gridline(
                Orientation::Vertical,
                column,
                column == 0 || column == column_count || column % 5 == 0,
                &grid,
                grid_type,
                size,
            ))
            .insert(grid_type)
            .id();
        lines.push(line);
    }
    commands.entity(parent).push_children(&lines);

    parent
}

fn build_gridline(
    orientation: Orientation,
    i: u32,
    edge: bool,
    data: &Res<GridData>,
    grid_type: GridType,
    size: (f32, f32),
) -> SpriteBundle {
    // The grid lines that define the boxes need to be thicker
    let thickness = if edge {
        data.line_thickness_bold
    } else {
        data.line_thickness
    };
    let left_edge = size.0 * -0.5;
    let bot_edge = size.1 * -0.5;

    let (x, y, sprite_size) = match orientation {
        Orientation::Horizontal => (
            0.0,
            left_edge + (i as f32 * data.cell_size),
            Vec2::new(size.1 as f32 + thickness, thickness),
        ),
        Orientation::Vertical => (
            bot_edge + (i as f32 * data.cell_size),
            0.0,
            Vec2::new(thickness, size.0 as f32 + thickness),
        ),
    };

    SpriteBundle {
        sprite: Sprite::new(sprite_size),
        transform: Transform::from_xyz(x, y, 0.0),
        material: match grid_type {
            GridType::X => data.grid_x_material.clone(),
            GridType::Y => data.grid_y_material.clone(),
            GridType::Z => data.grid_z_material.clone(),
        },
        ..Default::default()
    }
}
