use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::movement::Movable;


#[derive(Component)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub occupied: bool,
}

#[derive(Component)]
pub struct MovingRoad;

pub fn setup_road(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let grid_size_x = 1;
    let grid_size_y = 10;
    let tile_size = 32.0; 
    
    // Centrér vejen (svarer til offset_x logikken i grid.rs)
    let road_position_x = 0.0;

    for y in 0..grid_size_y {
        commands.spawn((
            Sprite {
                image: asset_server.load("tileset/stright_road.png"),
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                anchor: Anchor::Center,
                ..default()
            },
            Transform::from_xyz(
                road_position_x,
                y as f32 * tile_size - (grid_size_y as f32 * tile_size) / 3.5,
                1.0,
            ),
            Tile {
                x: 0,
                y: y as u32,
                occupied: false,
            },
            MovingRoad,
            Movable { speed: 50.0 },
        ));
    }
}

pub fn move_road(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Transform, &Tile), With<MovingRoad>>,
) {
    let tile_size = 32.0;
    let road_position_x = 0.0; // Centreret vejposition
    
    // Flag til at tjekke om den nederste flise er nået bunden
    let mut road_at_bottom = false;
    let mut highest_y = -1000.0; // Meget lav startværdi
    
    for (entity, mut transform, tile) in query.iter_mut() {
        // Find den højeste y-værdi blandt de resterende vejfliser
        if transform.translation.y > highest_y {
            highest_y = transform.translation.y;
        }
        
        // Hvis den nederste vejflise er nået bunden
        if transform.translation.y < -320.0 {
            road_at_bottom = true;
            commands.entity(entity).despawn();
        }
    }
    
    // Hvis vejen har nået bunden eller der er plads til flere fliser i toppen
    if road_at_bottom || highest_y < 240.0 {
        let offset_y = highest_y + tile_size;
        
        // Spawn 10 vejfliser i toppen, hver 1 flise bred
        for y in 0..10 {
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/stright_road.png"),
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    anchor: Anchor::Center,
                    ..default()
                },
                Transform::from_xyz(
                    road_position_x,
                    offset_y + y as f32 * tile_size,
                    1.0,
                ),
                Tile {
                    x: 0,
                    y: y as u32,
                    occupied: false,
                },
                MovingRoad,
                Movable { speed: 50.0 },
            ));
        }
    }
}