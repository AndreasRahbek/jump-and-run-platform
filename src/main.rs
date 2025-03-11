use bevy::prelude::*;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}