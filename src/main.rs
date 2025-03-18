mod character_spawn;
mod tileset;
mod camera;
mod grid;
mod moving_road;

use bevy::prelude::*;
use character_spawn::*;
use tileset::{setup_map};
use camera::*;
use grid::*;
use moving_road::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))// prevents blurry sprites
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0,TimerMode::Repeating)))
        .add_systems(Startup, (setup_map, setup_character, setup_camera, setup_grid, setup_road))
        .add_systems(Update, (animate_sprite, move_character, spawn_log, move_log, move_road))
        .run();

}







