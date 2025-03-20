mod character_spawn;
mod camera;
mod grid;
mod moving_road;
mod log;
mod movement;
mod collision;

use bevy::prelude::*;
use character_spawn::*;
use camera::*;
use grid::*;
use moving_road::*;
use collision::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))// prevents blurry sprites
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0,TimerMode::Repeating)))
        .add_systems(Startup, ( setup_character, setup_camera, setup_grid, setup_road))
        .add_systems(Update, (
            animate_sprite,
            move_character,
            log::spawn_log,
            movement::move_entities,
            moving_road::move_road,
            check_collision,
        ))
        .run();

}







