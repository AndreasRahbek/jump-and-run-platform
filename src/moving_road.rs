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
    let grid_size_y = 200;
    let tile_size = 32.0; // Størrelse af hver tile i pixels

    for x in 0..grid_size_x {
        for y in 0..grid_size_y {
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/stright_road.png"),
                    texture_atlas: None,
                    color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                    flip_x: false,
                    flip_y: false,
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    rect: None,
                    anchor: Anchor::BottomCenter,
                    image_mode: SpriteImageMode::Auto,
                ..default()
                },
               
                

                Transform::from_xyz(
                    x as f32 * tile_size - (grid_size_x as f32 * tile_size) / 100.0,
                    y as f32 * tile_size - (grid_size_y as f32 * tile_size) / 3.5,
                    1.0,
                ),
                Tile {
                    x: x as u32,
                    y: y as u32,
                    occupied: false,
                },
                MovingRoad,
                Movable { speed: 50.0 },
            ));
        }
    }
}

pub fn move_road(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Transform, &Tile), With<MovingRoad>>,
) {
    let tile_size = 32.0;
    let grid_size_y = 10;
    
    for (mut transform, tile) in query.iter_mut() {
        // Hvis vejen går ud af skærmen, flyt den til toppen og opdater dens position
        if transform.translation.y < -320.0 {
            transform.translation.y += 320.0 + (grid_size_y as f32 * tile_size);
            
            // Spawn nyt vejstykke i toppen
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/stright_road.png"),
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x,
                    320.0 + (grid_size_y as f32 * tile_size),
                    1.0,
                ),
                Tile {
                    x: tile.x,
                    y: tile.y + grid_size_y,
                    occupied: false,
                },
                MovingRoad,
                Movable { speed: 50.0 },
            ));
        }
    }
}