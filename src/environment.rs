use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::movement::Movable;

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

pub fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let grid_size_y = 15;
    let tile_size = 32.0;
    let road_position_x = 0.0;
    let environment_offset = 64.0; // Afstand fra vejens midte til miljø-objekterne
    
    // Spawn miljø-objekter på begge sider af vejen
    for y in 0..grid_size_y {
        // Venstre side
        if y % 3 == 0 { // Placer objekter med mellemrum (hvert 3. felt)
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/cactus.png"), // Erstat med dine egne miljø-assets
                    custom_size: Some(Vec2::new(tile_size * 1.5, tile_size * 1.5)), // Lidt større end grid
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(
                    road_position_x - environment_offset,
                    y as f32 * tile_size,
                    0.5, // Z-værdi mellem grid (0.0) og vej (1.0)
                ),
                EnvironmentObject {
                    x: 0,
                    y: y as u32,
                    side: Side::Left,
                },
                Environment,
                Movable { speed: 50.0 },
            ));
        }
        
        // Højre side
        if (y + 1) % 3 == 0 { // Forskudt i forhold til venstre side
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/cactus.png"), // Erstat med dine egne miljø-assets
                    custom_size: Some(Vec2::new(tile_size * 1.2, tile_size * 1.2)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(
                    road_position_x + environment_offset,
                    y as f32 * tile_size,
                    0.5,
                ),
                EnvironmentObject {
                    x: 1,
                    y: y as u32,
                    side: Side::Right,
                },
                Environment,
                Movable { speed: 50.0 },
            ));
        }
    }
}

pub fn move_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Transform, &EnvironmentObject), With<Environment>>,
) {
    let tile_size = 32.0;
    let road_position_x = 0.0;
    let environment_offset = 64.0;
    
    // Flag til at tjekke om miljø-elementer er nået bunden
    let mut env_at_bottom = false;
    let mut highest_y = -1000.0;
    
    // Spor objekter på hver side
    let mut highest_left = -1000.0;
    let mut highest_right = -1000.0;
    
    for (entity, mut transform, env_obj) in query.iter_mut() {
        // Find den højeste y-værdi for miljø-objekter
        if transform.translation.y > highest_y {
            highest_y = transform.translation.y;
        }
        
        // Registrer højeste y-værdi for hver side
        match env_obj.side {
            Side::Left => {
                if transform.translation.y > highest_left {
                    highest_left = transform.translation.y;
                }
            },
            Side::Right => {
                if transform.translation.y > highest_right {
                    highest_right = transform.translation.y;
                }
            }
        }
        
        // Hvis objektet er nået bunden
        if transform.translation.y < -320.0 {
            env_at_bottom = true;
            commands.entity(entity).despawn();
        }
    }
    
    // Hvis miljø-objekter har nået bunden eller der er plads til flere i toppen
    if env_at_bottom || highest_y < 240.0 {
        let offset_y_left = highest_left + tile_size * 3.0; // Hver 3. felt
        let offset_y_right = highest_right + tile_size * 3.0;
        
        // Spawn nye miljø-objekter i toppen
        for i in 0..5 { // Spawn 5 nye objekter på hver side
            // Venstre side
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/tree.png"),
                    custom_size: Some(Vec2::new(tile_size * 1.5, tile_size * 1.5)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(
                    road_position_x - environment_offset,
                    offset_y_left + i as f32 * tile_size * 3.0, // Fordel objekterne
                    0.5,
                ),
                EnvironmentObject {
                    x: 0,
                    y: i as u32,
                    side: Side::Left,
                },
                Environment,
                Movable { speed: 50.0 },
            ));
            
            // Højre side - lettere forskudt
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/tree.png"),
                    custom_size: Some(Vec2::new(tile_size * 1.2, tile_size * 1.2)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(
                    road_position_x + environment_offset,
                    offset_y_right + (i as f32 + 1.5) * tile_size * 3.0, // Forskudt
                    0.5,
                ),
                EnvironmentObject {
                    x: 1,
                    y: i as u32,
                    side: Side::Right,
                },
                Environment,
                Movable { speed: 50.0 },
            ));
        }
    }
}
