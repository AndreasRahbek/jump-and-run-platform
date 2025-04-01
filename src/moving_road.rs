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
    let road_height = grid_config.grid_height;

    let total_height = grid_config.grid_height as f32 * tile_size;
    let start_y = -(total_height / 2.0) + tile_size / 2.0;

    let spacing_fix = 0.5; // overlap-mængde
    let road_image = asset_server.load("tileset/stright_road.png");

    for i in 0..road_height {
        let y = start_y + i as f32 * (tile_size - spacing_fix);

        commands.spawn((
            Sprite {
                image: road_image.clone(),
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                anchor: Anchor::Center,
                ..default()
            },
            Transform::from_xyz(0.0, y, ROAD_Z),
            MovingRoad,
            GridObject,
        ));
    }
}


pub fn update_road(
    mut commands: Commands,
    road_query: Query<&Transform, With<MovingRoad>>,
    grid_config: Res<GridConfig>,
    asset_server: Res<AssetServer>,
) {
    let tile_size = grid_config.tile_size;
    let road_image = asset_server.load("tileset/stright_road.png");

    let lowest_y = road_query
        .iter()
        .map(|t| t.translation.y)
        .reduce(f32::min)
        .unwrap_or(0.0);

    let highest_y = road_query
        .iter()
        .map(|t| t.translation.y)
        .reduce(f32::max)
        .unwrap_or(0.0);

    let spawn_threshold = -(grid_config.grid_height as f32) * tile_size / 2.0 + tile_size * 1.0;
    let max_height = (grid_config.grid_height as f32) * tile_size / 2.0;

    if lowest_y < spawn_threshold {
        let mut y = highest_y + tile_size;
        while y < max_height {
            commands.spawn((
                Sprite {
                    image: road_image.clone(),
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    anchor: Anchor::Center,
                    ..default()
                },
                Transform::from_xyz(0.0, y, ROAD_Z),
                MovingRoad,
                GridObject,
            ));
            y += tile_size;
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

