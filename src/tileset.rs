use bevy::prelude::*;
pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite {
        image: asset_server.load("tileset/background.png"),
        image_mode: SpriteImageMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.0,
            
        },
        ..default()
    });
}
