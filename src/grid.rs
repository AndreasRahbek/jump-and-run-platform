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
use crate::movement::Movable;

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
pub struct SpawnTimer(pub Timer);

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
        Sprite::from_image(image_handle),
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
) {
    let grid_size_x = 15; // Beholder samme bredde
    let grid_size_y = 15; // Beholder samme højde
    let tile_size = 32.0;
    
    // Beregn offset for at centrere grid'et
    let total_width = grid_size_x as f32 * tile_size;
    let offset_x = -total_width / 2.0 + tile_size / 2.0;
    
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
                    (y as f32 * tile_size),
                    0.0,
                ),
                TileGrid {
                    x: x as u32,
                    y: y as u32,
                    occupied: false,
                },
                Movable { speed: 50.0 },
                GAME_LAYERS,
            ));
        }
    }
}

pub fn spawn_log(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    time: Res<Time>, 
    mut timer: ResMut<SpawnTimer>
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Log,
            Sprite {
                image: asset_server.load("tileset/log.png"),
                ..default()
            },
            Transform::from_xyz(0.0, 75.0, 2.0),
            GAME_LAYERS, // Tilføj render layer
        ));
    }
}

pub fn move_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut param_set: ParamSet<(
        Query<(Entity, &mut Transform), With<Log>>,
        Query<(Entity, &mut Transform, &TileGrid)>,
    )>,
    time: Res<Time>,
    mut movement_tracker: ResMut<GridMovementTracker>
) {
    let movement_this_frame = 50.0 * time.delta_secs();
    
    // Afrund bevægelse til hele pixels
    let movement_this_frame = movement_this_frame.floor();
    
    // Bevæg Log ned og fjern dem hvis de er ude af skærmen
    for (entity, mut transform) in param_set.p0().iter_mut() {
        transform.translation.y -= movement_this_frame;
        
        // Snap til grid
        transform.translation.x = transform.translation.x.floor();
        transform.translation.y = transform.translation.y.floor();
        
        // Fjern logs der er kommet ud af skærmen
        if transform.translation.y < -320.0 {
            commands.entity(entity).despawn();
        }
    }

    let grid_size = 15;
    let tile_size = 32.0;
    
    // Opdater movement tracker
    movement_tracker.distance_moved += movement_this_frame;
    
    // Flag til at tjekke om et helt grid er nået bunden
    let mut grid_at_bottom = false;
    let mut highest_y = -1000.0; // Meget lav startværdi
    
    // Bevæg TileGrid ned
    for (entity, mut transform, tile) in param_set.p1().iter_mut() {
        transform.translation.y -= movement_this_frame;
        
        // Snap til grid
        transform.translation.x = transform.translation.x.floor();
        transform.translation.y = transform.translation.y.floor();
        
        // Find den højeste y-værdi blandt de resterende tiles
        if transform.translation.y > highest_y {
            highest_y = transform.translation.y;
        }
        
        // Hvis den nederste række af grid er nået bunden
        if transform.translation.y < -320.0 && tile.y == 0 {
            grid_at_bottom = true;
        }
        
        // Hvis tile går ud af skærmen, fjern den
        if transform.translation.y < -320.0 {
            commands.entity(entity).despawn();
        }
    }
    
    // Hvis grid'et har nået bunden eller der er plads til et nyt grid i toppen
    if grid_at_bottom || highest_y < 240.0 {
        // Vi skal sikre at det nye grid placeres præcist
        let exactly_tile_size = tile_size; // tile_size er allerede et helt tal (32.0)
        let offset_y = highest_y.floor() + exactly_tile_size;
        
        let grid_size_x = 15;
        let grid_size_y = 15;
        let total_width = grid_size_x as f32 * tile_size;
        let offset_x = -total_width / 2.0 + tile_size / 2.0;

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
                        offset_y,
                        0.0,
                    ),
                    TileGrid {
                        x: x as u32,
                        y: y as u32,
                        occupied: false,
                    },
                    Movable { speed: 50.0 },
                    GAME_LAYERS,
                ));
            }
        }
        
        // Nulstil tracker
        movement_tracker.distance_moved = 0.0;
    }
}

// Tilpasser kanvas størrelsesforhold ved vindue-ændringer
pub fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        if let Ok(mut projection) = query.get_single_mut() {
            let h_scale = event.width / RES_WIDTH as f32;
            let v_scale = event.height / RES_HEIGHT as f32;
            // Tilføj en zoom-faktor på 1.2 (20% mere zoom)
            let zoom_factor = 1.2;
            projection.scale = 1. / (h_scale.min(v_scale).floor().max(1.0) * zoom_factor);
        }
    }
}
