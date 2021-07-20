mod map;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;
use crate::helpers;
use crate::{helpers::cleanup_system, GameState};

use map::MapPlugin;

#[derive(Inspectable, Debug)]
pub struct TanksData {
    clear_color: Color,

    board_material: Handle<ColorMaterial>,
}

impl FromWorld for TanksData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");

        TanksData {
            clear_color: Color::BLACK,
            board_material: materials.add(Color::BLACK.into()),
        }
    }
}

pub struct TanksPlugin {
    state: GameState,
}

impl TanksPlugin {
    pub fn new( state: GameState ) -> Self {
        TanksPlugin {
            state: state,
        }
    }
}

impl Plugin for TanksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<TanksData>::new().open(false))
            .add_plugin(MapPlugin::new(self.state.clone()))
            .add_system_set(
                SystemSet::on_enter(self.state.clone())
                .with_system(startup.system())
            )
            .add_system_set(
                SystemSet::on_update(self.state.clone())
                .with_system(update.system())
                .with_system(helpers::camera::movement.system())
            )
            .add_system_set(
                SystemSet::on_exit(self.state.clone())
                    .with_system(cleanup_system::<TankCleanup>.system()),
            );
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct TankCleanup;

fn startup(
    mut commands: Commands,
    data: Res<TanksData>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = data.clear_color;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation = Vec3::new(0.0, 0.0, -100.0);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(TankCleanup);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(TankCleanup);

}


fn update(
    mut clear_color: ResMut<ClearColor>,
    data: Res<TanksData>,
) {
    if data.is_changed() {
        clear_color.0 = data.clear_color;
    }
}


