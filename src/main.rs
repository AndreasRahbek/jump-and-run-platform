mod character_spawn;
mod tileset;
mod camera;
mod grid;

use bevy::prelude::*;
use character_spawn::*;
use tileset::{setup_map};
use camera::*;
use grid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))// prevents blurry sprites
        .add_systems(Startup, (setup_map, setup_character, setup_camera, setup_grid))
        .add_systems(Update, animate_sprite)
        .run();

}







