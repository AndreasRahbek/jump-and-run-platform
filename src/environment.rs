use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::world_grid::{GridObject, ENVIRONMENT_Z, GridConfig};

#[derive(Component)]
pub struct EnvironmentObject {
    pub x: u32,
    pub y: u32,
    pub side: Side, // Venstre eller højre side af vejen
}

#[derive(Component, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Environment;

// Define a constant for environment object speed
const ENVIRONMENT_SPEED: f32 = 50.0;

// In setup_environment function, add GridObject to both spawns
pub fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
) {
    let grid_size_y = grid_config.grid_height;
    let tile_size = grid_config.tile_size;
    let road_position_x = 0.0;
    let environment_offset = 64.0; // Afstand fra vejens midte til miljø-objekterne
    
    // Calculate the total height of the grid
    let total_height = grid_size_y as f32 * tile_size;
    // Calculate the starting position (top of the grid)
    let start_y = total_height / 2.0;
    
    // Spawn miljø-objekter på begge sider af vejen
    for y in 0..grid_size_y {
        // Venstre side
        if y % 3 == 0 { // Placer objekter med mellemrum (hvert 3. felt)
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/cactus.png"),
                    custom_size: Some(Vec2::new(tile_size, tile_size)), // Match tile size exactly
                    anchor: Anchor::Center, // Center anchor instead of BottomCenter
                    ..default()
                },
                Transform::from_xyz(
                    road_position_x - environment_offset,
                    start_y - (y as f32 * tile_size) - (tile_size / 2.0), // Center in tile
                    ENVIRONMENT_Z,
                ),
                EnvironmentObject {
                    x: 0,
                    y: y as u32,
                    side: Side::Left,
                },
                Environment,
                GridObject,
            ));
        }
        
        // Højre side
        if (y + 1) % 3 == 0 { // Forskudt i forhold til venstre side
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/cactus.png"),
                    custom_size: Some(Vec2::new(tile_size, tile_size)), // Match tile size exactly
                    anchor: Anchor::Center, // Center anchor instead of BottomCenter
                    ..default()
                },
                Transform::from_xyz(
                    road_position_x + environment_offset,
                    start_y - (y as f32 * tile_size) - (tile_size / 2.0), // Center in tile
                    ENVIRONMENT_Z,
                ),
                EnvironmentObject {
                    x: 1,
                    y: y as u32,
                    side: Side::Right,
                },
                Environment,
                GridObject,
            ));
        }
    }
}

