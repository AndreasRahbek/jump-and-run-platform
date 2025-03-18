mod character_run;
mod tileset;

use bevy::prelude::*;
use character_run::{animate_sprite, setup};
use tileset::{setupMap};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))// prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, setupMap)
        .run();

}







