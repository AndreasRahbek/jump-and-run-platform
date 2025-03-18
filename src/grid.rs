use bevy::prelude::*;
use bevy::sprite::Anchor;


#[derive(Component)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub occupied: bool,
}

pub fn setup_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let grid_size = 10; // 10x10 grid
    let tile_size = 32.0; // Størrelse af hver tile i pixels

    for x in 0..grid_size {
        for y in 0..grid_size {
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/background.png"),
                    texture_atlas: None,
                    color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                    flip_x: false,
                    flip_y: false,
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    rect: None,
                    anchor: Anchor::Center,
                    image_mode: SpriteImageMode::Auto,
                ..default()
                },
               
                

                Transform::from_xyz(
                    x as f32 * tile_size - (grid_size as f32 * tile_size) / 2.0,
                    y as f32 * tile_size - (grid_size as f32 * tile_size) / 2.0,
                    0.0,
                ),
                Tile {
                    x: x as u32,
                    y: y as u32,
                    occupied: false,
                },
            ));
        }
    }
}