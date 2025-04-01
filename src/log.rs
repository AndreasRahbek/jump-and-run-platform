use bevy::prelude::*;
use crate::collision::Collider;
use crate::background::*;
use crate::world_grid::{GridObject, LOG_Z};

#[derive(Component)]
pub struct Log;
#[derive(Resource)]
pub struct SpawnTimer(pub Timer);



pub fn spawn_log(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    timer.0.tick(time.delta());
    
    if timer.0.just_finished() {
        // Use a fixed x position for all logs to spawn in a single line
        let x_position = 0.0; // Center of the screen
        
        commands.spawn((
            Sprite {
                image: asset_server.load("tileset/log.png"),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(x_position, 250.0, LOG_Z),
            Log,
            Collider { 
                size: Vec2::new(16.0, 5.0),
                //is_trigger: false, // Add this field
            },
            GridObject,
            GAME_LAYERS,
        ));
    }
}