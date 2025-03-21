mod character;
mod camera;
mod grid;
mod moving_road;
mod log;
mod movement;
mod collision;

use bevy::prelude::*;
use character::*;
use camera::*;
use grid::*;
use moving_road::*;
use collision::*;
use movement::*;
use log::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))// prevents blurry sprites
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(JumpTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, (setup_pixel_grid, setup_character, setup_grid, setup_road))
        .add_systems(Update, (
            animate_sprite,
            move_character,
            jump,
            move_entities,
            move_road,
            check_collision,
            spawn_log,
            move_entities,
            move_map,
            move_road,
            fit_canvas,
        ))
        .run();

}







