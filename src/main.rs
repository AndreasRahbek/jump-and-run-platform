mod character_spawn;
mod camera;
mod grid;
mod moving_road;
mod log;
mod movement;
mod collision;
//mod environment;

use bevy::prelude::*;
use character_spawn::*;
use camera::*;
use grid::*;
use moving_road::*;
//use environment::*;
//use collision::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))// prevents blurry sprites
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0,TimerMode::Repeating)))
        .add_systems(Startup, (
            setup_pixel_grid,  // Tilføj først for at opsætte kamera
            setup_character, 
            setup_grid, 
            setup_road,
            //setup_environment,
        ))
        .add_systems(Update, (
            animate_sprite,
            move_character,
            log::spawn_log,
            movement::move_entities,
            move_map,  // Tilføj din egen bevægelsessystem
            moving_road::move_road,
            //environment::move_environment,
            fit_canvas,  // Tilføj for at håndtere vindue-ændringer
        ))
        .run();

}







