use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::world_grid::{GridObject, ENVIRONMENT_Z, GridConfig};
use rand::Rng;

#[derive(Component)]
pub struct EnvironmentObject {
    pub side: Side, // Venstre eller højre side af vejen
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Environment;

pub fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
) {
    let grid_size_y = grid_config.grid_height;
    let tile_size = grid_config.tile_size;
    
    let total_height = grid_size_y as f32 * tile_size;
    let start_y = total_height / 2.0;
    
    let mut rng = rand::thread_rng();

    for y in 0..grid_size_y {
        let current_y = start_y - (y as f32 * tile_size) - (tile_size / 2.0);

        // Random chance for left side
        if rng.gen_bool(0.15) {
            let offset = rng.gen_range(60.0..100.0);
            spawn_environment_object(
                &mut commands,
                &asset_server,
                -offset,
                current_y,
                tile_size,
                Side::Left,
                &mut rng
            );
        }
        
        // Random chance for right side
        if rng.gen_bool(0.15) {
            let offset = rng.gen_range(60.0..100.0);
            spawn_environment_object(
                &mut commands,
                &asset_server,
                offset,
                current_y,
                tile_size,
                Side::Right,
                &mut rng
            );
        }
    }
}

pub fn update_environment(
    mut commands: Commands,
    query: Query<&Transform, With<EnvironmentObject>>,
    grid_config: Res<GridConfig>,
    asset_server: Res<AssetServer>,
) {
    let tile_size = grid_config.tile_size;
    
    let max_height = (grid_config.grid_height as f32) * tile_size / 2.0;

    let highest_y = query.iter()
        .map(|t| t.translation.y)
        .reduce(f32::max)
        .unwrap_or(-1000.0); // Fallback if no objects exist

    // If we have space at the top, spawn new objects
    if highest_y < max_height {
        let mut rng = rand::thread_rng();
        let mut current_y = highest_y + tile_size;

        while current_y < max_height + tile_size { // Buffer to ensure coverage
            
            // Reduced probability for more spacing/randomness (15%)
            if rng.gen_bool(0.15) {
                 let offset = rng.gen_range(60.0..100.0);
                 spawn_environment_object(
                    &mut commands,
                    &asset_server,
                    -offset,
                    current_y,
                    tile_size,
                    Side::Left,
                    &mut rng
                );
            }

            if rng.gen_bool(0.15) {
                 let offset = rng.gen_range(60.0..100.0);
                 spawn_environment_object(
                    &mut commands,
                    &asset_server,
                    offset,
                    current_y,
                    tile_size,
                    Side::Right,
                    &mut rng
                );
            }

            current_y += tile_size;
        }
    }
}

pub fn remove_old_environment(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<EnvironmentObject>>,
    grid_config: Res<GridConfig>,
) {
    let despawn_threshold = -(grid_config.grid_height as f32) * grid_config.tile_size;

    for (entity, transform) in query.iter() {
        if transform.translation.y < despawn_threshold {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_environment_object(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
    size: f32,
    side: Side,
    rng: &mut rand::rngs::ThreadRng,
) {
    let is_cactus = rng.gen_bool(0.5);
    let texture_path = if is_cactus { "tileset/cactus.png" } else { "tileset/tree.png" };

    commands.spawn((
        Sprite {
            image: asset_server.load(texture_path),
            custom_size: Some(Vec2::new(size, size)),
            anchor: Anchor::Center,
            ..default()
        },
        Transform::from_xyz(x, y, ENVIRONMENT_Z),
        EnvironmentObject { side },
        Environment,
        GridObject,
    ));
}
