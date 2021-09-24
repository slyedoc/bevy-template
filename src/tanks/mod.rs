mod map;
pub mod actions;
pub mod camera;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;
use crate::GameState;

use map::MapPlugin;
use camera::CameraPlugin;

#[derive(Inspectable, Debug)]
pub struct TanksData {
    clear_color: Color,
}

impl FromWorld for TanksData {
    fn from_world(world: &mut World) -> Self {
        TanksData {
            clear_color: Color::BLACK,
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
            .add_plugin(CameraPlugin::new(self.state.clone()))
            .add_system_set(
                SystemSet::on_enter(self.state.clone())
                .with_system(startup.system())
            )
            .add_system_set(
                SystemSet::on_update(self.state.clone())
                .with_system(update.system())
            );
    }
}


fn startup(
    data: Res<TanksData>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = data.clear_color;
}

fn update(
    mut clear_color: ResMut<ClearColor>,
    data: Res<TanksData>,
) {
    if data.is_changed() {
        clear_color.0 = data.clear_color;
    }
}


