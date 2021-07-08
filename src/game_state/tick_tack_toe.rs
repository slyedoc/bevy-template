use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;

use crate::{helpers::cleanup_system, GameState};

#[derive(Inspectable, Debug)]
pub struct TicTackToeData {
    clear_color: Color,

    #[inspectable(min = 0.0, max = 300.0, label = "Size")]
    size: f32,

    #[inspectable(min = 0.01, max = 20.0, label = "Line Thickness")]
    line_thickness: f32,

    board_material: Handle<ColorMaterial>,
    x_material: Handle<ColorMaterial>,
    o_material: Handle<ColorMaterial>,
}

impl FromWorld for TicTackToeData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");


        TicTackToeData {
            clear_color: Color::WHITE,
            board_material: materials.add(Color::BLACK.into()),
            o_material: materials.add(Color::BLUE.into()),
            x_material: materials.add(Color::RED.into()),
            size: 300.0,
            line_thickness: 10.0,
        }
    }
}

pub struct TicTackToePlugin;

impl Plugin for TicTackToePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<TicTackToeData>::new().open(false))
            .add_system_set(SystemSet::on_enter(GameState::TicTackToe).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(GameState::TicTackToe).with_system(update.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::TicTackToe)
                    .with_system(cleanup_system::<TicTackToe>.system()),
            );
    }
}

enum TicTackToe {
    Board,
}

fn setup(mut commands: Commands, data: Res<TicTackToeData>) {
    println!("setup_tic_tack_toe");

    let board_center_offset = Vec2::new(0.0, 0.0);
    let board_left_edge: f32 = board_center_offset.x - 0.5 * data.size;
    let board_bot_edge: f32 = board_center_offset.y - 0.5 * data.size;

    let offset = data.size / 3.0;

    // Horizontal
    commands
        .spawn_bundle(draw_line(
            Vec2::new(data.line_thickness, data.size),
            Vec2::new(board_left_edge + offset, 0.0),
            &data,
        ))
        .insert(TicTackToe::Board);

    commands
        .spawn_bundle(draw_line(
            Vec2::new(data.line_thickness, data.size),
            Vec2::new(board_left_edge + (offset * 2.0), 0.0),
            &data,
        ))
        .insert(TicTackToe::Board);

    // Vertical
    commands
        .spawn_bundle(draw_line(
            Vec2::new( data.size,data.line_thickness,),
            Vec2::new( 0.0, board_left_edge + offset,),
            &data,
        ))
        .insert(TicTackToe::Board);

    commands
        .spawn_bundle(draw_line(
            Vec2::new( data.size,data.line_thickness,),
            Vec2::new( 0.0, board_left_edge + (offset * 2.0)),
            &data,
        ))
        .insert(TicTackToe::Board);
}

fn draw_line(size: Vec2, pos: Vec2, data: &Res<TicTackToeData>) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            size: size,
            ..Default::default()
        },
        material: data.board_material.clone(),
        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
        ..Default::default()
    }
}

fn update(
    mut clear_color: ResMut<ClearColor>,
    data: Res<TicTackToeData>
) {
    clear_color.0 = data.clear_color;
}