use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, widgets::InspectableButton};
use super::*;

#[derive(Inspectable, Debug)]
pub struct GridData {

    #[inspectable()]
    pub size: (u32, u32, u32),

    pub show_x_grid: bool,
    pub show_y_grid: bool,
    pub show_z_grid: bool,

    pub grid_center: bool,
    #[inspectable(min = 0.0)]
    pub cell_size: f32,

    pub grid_x_material: Handle<ColorMaterial>,
    pub grid_y_material: Handle<ColorMaterial>,
    pub grid_z_material: Handle<ColorMaterial>,

    #[inspectable(min = 0.0, max = 10.0)]
    pub line_thickness: f32,
    #[inspectable(min = 0.0, max = 10.0)]
    pub line_thickness_bold: f32,

    #[inspectable(text = "Rebuild")]
    rebuild: InspectableButton<GridResetEventButton>
}



impl FromWorld for GridData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");

        let size = 500;
        GridData {
            size: (size, size, size),
            show_x_grid: false,
            show_y_grid: true,
            show_z_grid: false,
            grid_center: true,
            grid_x_material: materials.add(Color::RED.into()),
            grid_y_material: materials.add(Color::GREEN.into()),
            grid_z_material: materials.add(Color::BLUE.into()),
            line_thickness: 0.01,
            line_thickness_bold: 0.3,
            cell_size: 10.0,
            rebuild: InspectableButton::<GridResetEventButton>::new()
        }
    }
}