pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(InspectorPlugin::<Inspector>::new().open(true).shared())
        .add_system_to_stage(
            CoreStage::PostUpdate,
            maintain_inspected_entities
                .system()
                .after(bevy_mod_picking::PickingSystem::Focus),
        );
    }
}

#[derive(Inspectable, Default)]
struct Inspector {
    #[inspectable(deletable = false)]
    active: Option<Entity>,
}

fn maintain_inspected_entities(
    mut inspector: ResMut<Inspector>,
    query: Query<(Entity, &Transform, &Interaction), Changed<Interaction>>,
    mut camera: Query<&mut PanOrbitCamera>,
) {
    let mut transform: &Transform = &Transform::from_xyz(0.0, 0.0, 0.0);
    let entity = query
        .iter()
        .filter(|(_, _, interaction)| matches!(interaction, Interaction::Clicked))
        .map(|(entity, t, _)| {
            transform = t;
            entity
        })
        .next();

    if let Some(entity) = entity {
        // Deselect if already selected
        if inspector.active == Some(entity) {
            inspector.active = None;
        } else {
            inspector.active = Some(entity);

            for mut camera in camera.iter_mut() {
                camera.focus = transform.translation;
            }
        }
    }
}
