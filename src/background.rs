use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::render::{
    camera::RenderTarget,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    view::RenderLayers,
};
use bevy::window::WindowResized;
use crate::world_grid::{GridObject, GridConfig, GRID_Z};

// Spil opløsning - justér efter behov
const RES_WIDTH: u32 = 480;  // 15 * 32
const RES_HEIGHT: u32 = 480; // 15 * 32

// Render layers
pub const GAME_LAYERS: RenderLayers = RenderLayers::layer(0);
pub const UI_LAYERS: RenderLayers = RenderLayers::layer(1);

#[derive(Component)]
pub struct TileGrid {
    pub x: u32,
    pub y: u32,
    pub occupied: bool,
}

#[derive(Component)]
pub struct Log;

#[derive(Resource)]
pub struct GridMovementTracker {
    pub distance_moved: f32,
    pub threshold: f32,
}

#[derive(Component)]
pub struct GameCanvas;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct OuterCamera;

// Opsætter kamera og kanvas for pixel-perfect rendering
pub fn setup_pixel_grid(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // Opret kanvas med fast opløsning
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    canvas.resize(canvas_size);
    let image_handle = images.add(canvas);

    // Indre kamera der renderer spillet til kanvas
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            target: RenderTarget::Image(image_handle.clone().into()),
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        },
        Msaa::Off,
        GameCamera,
        GAME_LAYERS,
    ));

    // Kanvas sprite
    commands.spawn((
        Sprite {
            image: image_handle,
            ..default()
        },
        Transform::default(),
        GameCanvas,
        UI_LAYERS,
    ));

    // Ydre kamera der renderer kanvas til skærmen
    commands.spawn((
        Camera2d, 
        Msaa::Off, 
        OuterCamera, 
        UI_LAYERS
    ));
    
    // Tilføj ressource til at holde styr på bevægelse
    commands.insert_resource(GridMovementTracker { 
        distance_moved: 0.0, 
        threshold: 32.0 * 10.0 
    });
}

pub fn setup_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
) {
    let grid_size_x = grid_config.grid_width;
    let grid_size_y = grid_config.grid_height;
    let tile_size = grid_config.tile_size;
    
    // Beregn offset for at centrere grid'et
    let total_width = grid_size_x as f32 * tile_size;
    let total_height = grid_size_y as f32 * tile_size;
    let offset_x = -total_width / 2.0 + tile_size / 2.0;
    
    // Calculate the starting position (top-left corner of the grid)
    // This matches the calculation in draw_grid_lines
    let start_y = total_height / 2.0;
    
    for x in 0..grid_size_x {
        for y in 0..grid_size_y {
            commands.spawn((
                Sprite {
                    image: asset_server.load("tileset/background.png"),
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    anchor: Anchor::TopCenter,
                    ..default()
                },
                Transform::from_xyz(
                    offset_x + (x as f32 * tile_size),
                    start_y - (y as f32 * tile_size), // Position from the top
                    GRID_Z,
                ),
                TileGrid {
                    x: x as u32,
                    y: y as u32,
                    occupied: false,
                },
                GridObject, // Add this component
                GAME_LAYERS,
            ));
        }
    }
}

// Make sure this function is public
pub fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        if let Ok(mut projection) = query.get_single_mut() {
            let h_scale = event.width / RES_WIDTH as f32;
            let v_scale = event.height / RES_HEIGHT as f32;

            let zoom_factor = 3.5;
            projection.scale = 1. / (h_scale.min(v_scale).floor().max(1.0) * zoom_factor);
        }
    }
}

pub fn update_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
    mut tile_query: Query<(Entity, &Transform), With<TileGrid>>,
) {
    let tile_size = grid_config.tile_size;
    let grid_size_x = grid_config.grid_width;
    let grid_size_y = grid_config.grid_height;

    let total_height = grid_size_y as f32 * tile_size;
    let offset_x = -(grid_size_x as f32) * tile_size / 2.0 + tile_size / 2.0;

    // Find den laveste y-position af baggrundsfliserne
    let mut lowest_y = f32::MAX;
    for (_, transform) in tile_query.iter() {
        if transform.translation.y < lowest_y {
            lowest_y = transform.translation.y;
        }
    }

    // Hvis den nederste række er ved at forlade skærmen, spawn en ny række øverst
    let spawn_threshold = -total_height / 2.0;
    if lowest_y < spawn_threshold {
        let highest_y = lowest_y + total_height;

        for x in 0..grid_size_x {
            for y in 0..grid_size_y {
                commands.spawn((
                    Sprite {
                        image: asset_server.load("tileset/background.png"),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        anchor: Anchor::TopCenter,
                        ..default()
                    },
                    Transform::from_xyz(
                        offset_x + (x as f32 * tile_size),
                        highest_y - (y as f32 * tile_size),
                        GRID_Z,
                    ),
                    TileGrid {
                        x: x as u32,
                        y: y as u32,
                        occupied: false,
                    },
                    GridObject,
                    GAME_LAYERS,
                ));
            }
        }
    }
}

pub fn remove_old_background(
    mut commands: Commands,
    tile_query: Query<(Entity, &Transform), With<TileGrid>>,
    grid_config: Res<GridConfig>,
) {
    let despawn_threshold = -(grid_config.grid_height as f32) * grid_config.tile_size;

    for (entity, transform) in tile_query.iter() {
        if transform.translation.y < despawn_threshold {
            commands.entity(entity).despawn();
        }
    }
}