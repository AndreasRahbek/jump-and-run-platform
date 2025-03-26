mod character;
mod camera;
mod grid;
mod moving_road;
mod log;
mod movement;
mod collision;
mod environment;
mod world_grid;
use bevy::prelude::*;
use character::*;
use grid::*; 
use moving_road::*;
use collision::*;
use log::*;
use environment::*;
use world_grid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))// prevents blurry sprites
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(JumpTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, (
            setup_world_grid,
            setup_pixel_grid,
            setup_grid, 
            setup_road,
            setup_environment,
            setup_character,
        ).chain())
        .add_systems(Update, (
            world_grid::toggle_grid_debug,
            animate_sprite,
            move_character_horizontal,
            jump,
            move_grid_objects,
            check_collision,
            spawn_log,
            check_offscreen_objects,
            grid::fit_canvas,
            update_debug_grid.after(move_grid_objects)
        ))
        .run();
}








