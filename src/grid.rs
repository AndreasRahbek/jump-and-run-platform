use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::movement::Movable;


#[derive(Component)]
pub struct TileGrid {
    pub x: u32,
    pub y: u32,
    pub occupied: bool,
}

#[derive(Component)]
pub struct Log;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn setup_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let grid_size = 15; // 10x10 grid
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
                    anchor: Anchor::TopCenter,
                    image_mode: SpriteImageMode::Auto,
                ..default()
                },
               
                

                Transform::from_xyz(
                    x as f32 * tile_size - (grid_size as f32 * tile_size) / 2.0,
                    y as f32 * tile_size - (grid_size as f32 * tile_size) / 2.0,
                    0.0,
                ),
                TileGrid {
                    x: x as u32,
                    y: y as u32,
                    occupied: false,
                },
                Movable { speed: 50.0 },
            ));
        }
    }
}

pub fn spawn_log(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Log, // Log-komponenten
            Sprite {
                image: asset_server.load("tileset/log.png"),
                ..default()
            },
            Transform::from_xyz(0.0, 75.0, 2.0), // Position i verden
        ));
    }
}

pub fn move_map(
    mut param_set: ParamSet<(
    Query<(&mut Transform, &Log)>,  // Query for Log
    Query<(&mut Transform, &TileGrid)>,  // Query for TileGrid
    )>,
    time: Res<Time>)  {

    // Bevæg Log ned
    for (mut transform, _log) in param_set.p0().iter_mut() {
        transform.translation.y -= 50.0 * time.delta_secs();  // Flyt log ned
    }

    // Bevæg TileGrid ned
    for (mut transform, _tile) in param_set.p1().iter_mut() {
        transform.translation.y -= 50.0 * time.delta_secs();  // Flyt tile ned
    }
}