use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::movement::Movable;
use crate::world_grid::{GridObject, GridConfig, ROAD_Z};

#[derive(Component)]
pub struct MovingRoad;

pub fn setup_road(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
) {
    let tile_size = grid_config.tile_size;
    let road_height = 10; // Number of road tiles to stack vertically
    
    // Calculate the total height of the grid
    let total_height = grid_config.grid_height as f32 * tile_size;
    // Calculate the starting position (top of the grid)
    let start_y = total_height / 2.0;
    
    // Load the road tile image once
    let road_image = asset_server.load("tileset/stright_road.png");
    
    // Spawn road tiles stacked vertically
    for i in 0..road_height {
        commands.spawn((
            Sprite {
                image: road_image.clone(),
                custom_size: Some(Vec2::new(tile_size, tile_size)), // Each tile is square
                anchor: Anchor::Center, // Center anchor for better alignment
                ..default()
            },
            Transform::from_xyz(
                0.0,                // Centered horizontally
                start_y - (i as f32 * tile_size) - (tile_size / 2.0), // Position from the top, centered in tile
                ROAD_Z,
            ),
            MovingRoad,
            Movable { speed: 50.0 },
            GridObject, // This component ensures it moves with the grid system
        ));
    }
}
