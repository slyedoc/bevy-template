
impl MapPlugin {
    pub fn new(state: GameState) -> Self {
        MapPlugin { state: state }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(self.state.clone()).with_system(startup.system())
        ).add_system_set(
            SystemSet::on_enter(self.state.clone()).with_system(cleanup_system::<MapCleanup>.system())
        );
    }
}