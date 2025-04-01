mod character;
mod background;
mod moving_road;
mod log;
mod collision;
mod environment;
mod world_grid;
mod microbit;

use bevy::prelude::*;
use character::*;
use background::*;
use moving_road::*;
use collision::*;
use log::*;
use environment::*;
use world_grid::*;
use crate::microbit::{setup_serial_listener, JumpSignal};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .insert_resource(JumpSignal::default())
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(JumpTimer(Timer::from_seconds(0.5, TimerMode::Once)))
        .add_systems(Startup, (
            setup_world_grid,
            setup_serial_listener,
            setup_pixel_grid,
            setup_grid,
            setup_road,
            setup_environment,
            setup_character,
        ).chain())
        .add_systems(Update, (
            execute_animations,
            update_road,
            remove_old_road,
            update_background,
            move_character_horizontal,
            remove_old_background,
            animate_sprite,
            jump,
            move_grid_objects,
            check_collision,
            spawn_log,
            check_offscreen_objects,
            background::fit_canvas,
            //update_debug_grid.after(move_grid_objects)
        ))
        .run();
}








