use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;
use bevy_mod_picking::PickableBundle;

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
    none_material: Handle<ColorMaterial>,
}

#[derive(Inspectable, Default)]
struct Inspector {
    #[inspectable(deletable = false)]
    active: Option<Entity>,
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
            none_material: materials.add(Color::ANTIQUE_WHITE.into()),
            size: 400.0,
            line_thickness: 10.0,
        }
    }
}

pub struct TicTackToePlugin;

impl Plugin for TicTackToePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<TicTackToeData>::new().open(false))
            .add_system_set(SystemSet::on_enter(GameState::TicTackToe).with_system(setup.system()))
            .add_system_set(SystemSet::on_exit(GameState::TicTackToe).with_system(cleanup_system::<TicTackToe>.system()))
            .add_system_set(
                SystemSet::on_update(GameState::TicTackToe).with_system(update.system()),
            )
            .add_plugin(InspectorPlugin::<Inspector>::new())
            .add_system_to_stage(
        CoreStage::PostUpdate,
        maintain_inspected_entities
            .system()
            .after(bevy_mod_picking::PickingSystem::Focus),
    )        // PickingPlugin provides core picking systems and must be registered first
            ;
    }
}

fn maintain_inspected_entities(
    mut inspector: ResMut<Inspector>,
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
) {
    let entity = query
        .iter()
        .filter(|(_, interaction)| matches!(interaction, Interaction::Clicked))
        .map(|(entity, _)| entity)
        .next();

    if let Some(entity) = entity {
        if inspector.active == Some(entity) {
            inspector.active = None;
        } else {
            inspector.active = Some(entity);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TicTackToe {
    Board,
    Cell,
    Camera,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    None,
    X,
    O,
}

fn setup(mut commands: Commands, data: Res<TicTackToeData>) {

    let board_center_offset = Vec2::new(0.0, 0.0);
    let board_left_edge: f32 = board_center_offset.x - 0.5 * data.size;
    let board_bot_edge: f32 = board_center_offset.y - 0.5 * data.size;

    let offset = data.size / 3.0;

    // Spawn Camera
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(TicTackToe::Camera);

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(TicTackToe::Camera);

    //Board Horizontal
    draw_line(
        &mut commands,
        Vec2::new(data.line_thickness, data.size),
        Vec2::new(board_left_edge + offset, 0.0),
        &data.board_material,
    );

    draw_line(
        &mut commands,
        Vec2::new(data.line_thickness, data.size),
        Vec2::new(board_left_edge + (offset * 2.0), 0.0),
        &data.board_material,
    );

    // Board Vertical
    draw_line(
        &mut commands,
        Vec2::new(data.size, data.line_thickness),
        Vec2::new(0.0, board_left_edge + offset),
        &data.board_material,
    );

    draw_line(
        &mut commands,
        Vec2::new(data.size, data.line_thickness),
        Vec2::new(0.0, board_left_edge + (offset * 2.0)),
        &data.board_material,
    );

    // Board Cells
    for i in 0..=2 {
        for j in 0..=2 {
            draw_cell(
                &mut commands,
                Vec2::new(offset, offset),
                Vec3::new(
                    board_left_edge + (offset * 0.5) + offset * i as f32,
                    board_bot_edge + (offset * 0.5) + offset * j as f32,
                    0.0
                ),
                &data,
                Cell::None,
            );
        }
    }
}

fn draw_line<'a>(
    commands: &'a mut Commands,
    size: Vec2,
    pos: Vec2,
    material: &'a Handle<ColorMaterial>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                size: size,
                ..Default::default()
            },
            material: material.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..Default::default()
        })
        .insert(TicTackToe::Board);
}

fn draw_cell<'a>(
    commands: &'a mut Commands,
    size: Vec2,
    pos: Vec3,
    data: &'a Res<TicTackToeData>,
    cell_type: Cell,
) {
    let material = match cell_type {
        Cell::None => data.none_material.clone(),
        Cell::X => data.x_material.clone(),
        Cell::O => data.o_material.clone(),
    };

    let mut cell = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                size: size * 0.8,
                ..Default::default()
            },
            material,
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..Default::default()
        });

        cell.insert(TicTackToe::Cell)
            .insert(Cell::None);

        if cell_type == Cell::None {
            cell.insert_bundle(PickableBundle::default());
        }

}

fn update(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    data: Res<TicTackToeData>,
    query: Query<(Entity, &Sprite, &Transform,  &Interaction), (With<Cell>, Changed<Interaction>)>,
) {
    // TODO: Remove this hack, but it lets each state have its own background color
    clear_color.0 = data.clear_color;


    let (entity, size, pos) = query
    .iter()
    .filter(|(_, _, _, interaction)| matches!(interaction, Interaction::Clicked))
    .map(|(entity, s, t, _)| {
        (Some(entity), s.size,  t.translation)
    })
    .next()
    .unwrap_or_else(|| (None, Vec2::new(10.0, 10.0), Vec3::ZERO));


    if let Some(entity) = entity {
        commands.entity(entity).despawn_recursive();

        draw_cell(&mut commands, size, pos, &data, Cell::O);
    }
        //
}
