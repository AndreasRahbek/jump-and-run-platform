mod character_spawn;
mod tileset;

use bevy::prelude::*;
use character_spawn::{animate_sprite, setup_character};
use tileset::{setup_map};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))// prevents blurry sprites
        .add_systems(Startup, setup_character)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, setup_map)
        .run();

}







