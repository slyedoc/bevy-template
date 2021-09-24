use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable};
use bevy_mod_picking::{MeshButtonMaterials, PickableBundle, PickingEvent};
use rand::Rng;

use crate::{helpers::cleanup_system, GameState};

pub struct MapPlugin {
    state: GameState,
}

#[derive(Inspectable, Debug)]
struct MapData {
    pub cell: Handle<StandardMaterial>,
    pub hover: Handle<StandardMaterial>,
    pub selected: Handle<StandardMaterial>,
}

impl FromWorld for MapData {
    fn from_world(world: &mut World) -> Self {

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("ResMut<Assets<StandardMaterial>> not found.");

        let size = 10000;
        MapData {
            cell: materials.add(StandardMaterial {
                base_color: Color::rgb(0.7, 0.7, 0.7),
                // vary key PBR parameters on a grid of spheres to show the effect
                unlit: true,
                ..Default::default()
            }),
            hover: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.7, 0.0),
                // vary key PBR parameters on a grid of spheres to show the effect
                unlit: true,
                ..Default::default()
            }),
            selected: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.7, 0.0),
                // vary key PBR parameters on a grid of spheres to show the effect
                unlit: true,
                ..Default::default()
            })
        }
    }
}

struct MapCleanup;

impl MapPlugin {
    pub fn new(state: GameState) -> Self {
        MapPlugin { state: state }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(InspectorPlugin::<MapData>::new().open(false))
        .add_system_set(SystemSet::on_enter(self.state.clone()).with_system(startup.system()))
            .add_system_set(
                SystemSet::on_update(self.state.clone()).with_system(mouse_interactions.system()),
            )
            .add_system_set(
                SystemSet::on_enter(self.state.clone())
                    .with_system(cleanup_system::<MapCleanup>.system()),
            )
            .add_system_to_stage(CoreStage::PostUpdate, print_events.system());
    }
}

pub fn mouse_interactions(mut query: Query<(&mut Interaction, &mut Transform, &mut MineLocation), Changed<Interaction>>) {
    for (i, mut t, m) in query.iter_mut() {

        if *i == Interaction::Clicked {

        }

        if *i == Interaction::Hovered {
            let pos = t.up() * 50.0;
            t.translation += pos;
        }

        if *i == Interaction::None {
            t.translation = Vec3::new(t.translation.x, 0.0, t.translation.z);
        }
    }
}

pub fn print_events(mut events: EventReader<PickingEvent>, ) {
    for event in events.iter() {
        println!("This event happened! {:?}", event);
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    data: ResMut<MapData>,
    mut highlight_colors: ResMut<MeshButtonMaterials>,
) {
    let mut rnd = rand::thread_rng();
    highlight_colors.hovered = data.hover.clone();
    highlight_colors.selected = data.selected.clone();

    // spawn floor
    let rows = 10u32;
    let columns = 10u32;
    let size = 256.0;
    let origin_bottom_offset = -size * 0.5 * rows as f32;
    let origin_left_offset = -size * 0.5 * columns as f32;
    for x in 0..rows {
        for z in 0..columns {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: size })),
                    material: data.cell.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            origin_left_offset + (size * x as f32) + size * 0.5,
                            0.0,
                            origin_bottom_offset + size * z as f32 + size * 0.5,
                        ),
                        scale: Vec3::splat(0.99),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(MineLocation {
                    x: x,
                    y: z,
                    status: MineStatus::Unknown {
                        mine: rnd.gen_bool(0.10),
                    },
                })
                .insert_bundle(PickableBundle::default())
                .insert(MapCleanup);
        }
    }
}

#[allow(dead_code)]
pub enum MineStatus {
    Unknown { mine: bool },
    Count { count: u32 },
    Marked,
    Exploded,
}

#[allow(dead_code)]
pub struct MineLocation {
    status: MineStatus,
    x: u32,
    y: u32,
}
