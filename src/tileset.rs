use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasSprite};


pub fn setupMap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load the full spritesheet texture
    let texture_handle = asset_server.load("./assets/tileset/roads.png");

    // Define the spritesheet grid (128x64 with 32x32 individual sprites → 4 columns, 2 rows)
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 2, None, None);
    let texture_atlas_handle = atlases.add(texture_atlas);

    // Spawn sprites from the spritesheet
    for row in 0..2 {
        for col in 0..4 {
            let index = row * 4 + col; // Calculate index in the atlas
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(index),
                    transform: Transform::from_xyz(col as f32 * 40.0, row as f32 * -40.0, 0.0),
                    ..default()
                },
            ));
        }
    }

    // Spawn Camera
    commands.spawn(Camera2dBundle::default());
}
