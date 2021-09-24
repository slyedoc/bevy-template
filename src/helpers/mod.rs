pub mod camera;

use std::fmt::Debug;
use std::hash::Hash;

use bevy::{ecs::component::Component, prelude::*};
use bevy_input_actionmap::InputMap;



pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn cleanup_actions_system<T: 'static + Hash + Eq + Send + Sync + Debug + Clone >(mut input_map: ResMut<InputMap<T>>) {
    input_map.clear();
}

