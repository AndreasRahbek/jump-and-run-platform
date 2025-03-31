use bevy::prelude::*;
use bevy::sprite::Anchor;
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
            GridObject, // This component ensures it moves with the grid system
        ));
    }
}
pub fn update_road(
    mut commands: Commands,
    mut road_query: Query<(Entity, &mut Transform), With<MovingRoad>>,
    grid_config: Res<GridConfig>,
    asset_server: Res<AssetServer>,
) {
    let tile_size = grid_config.tile_size;
    let road_image = asset_server.load("tileset/stright_road.png");

    // Find den laveste og højeste flise
    let mut lowest_y = f32::MAX;
    let mut highest_existing_y = f32::MIN;

    for (_, transform) in road_query.iter() {
        if transform.translation.y < lowest_y {
            lowest_y = transform.translation.y - 10.;
        }
        if transform.translation.y > highest_existing_y {
            highest_existing_y = transform.translation.y;
        }
    }

    // Definer grænser
    let spawn_threshold = -(grid_config.grid_height as f32) * tile_size / 2.0 + tile_size; // Udenfor skærmen
    let max_height = (grid_config.grid_height as f32) * tile_size / 2.0; // Øverste grænse

    if lowest_y < spawn_threshold {
        let mut highest_y = lowest_y + (tile_size * 10.0);

        while highest_y < max_height {
            commands.spawn((
                Sprite {
                    image: road_image.clone(),
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    anchor: Anchor::Center,
                    ..default()
                },
                Transform::from_xyz(0.0, highest_y, ROAD_Z),
                MovingRoad,
                GridObject,
            ));
            highest_y += tile_size;
        }
    }
}

pub fn remove_old_road(
    mut commands: Commands,
    road_query: Query<(Entity, &Transform), With<MovingRoad>>,
    grid_config: Res<GridConfig>,
) {
    let despawn_threshold = -(grid_config.grid_height as f32) * grid_config.tile_size;

    for (entity, transform) in road_query.iter() {
        if transform.translation.y < despawn_threshold {
            commands.entity(entity).despawn();
        }
    }
}

