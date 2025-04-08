use bevy::prelude::*;
use crate::scoreboard::{ScoreText, ScoreTimer};

// Z-index constants for layering
pub const GRID_Z: f32 = 0.0;
pub const ROAD_Z: f32 = 1.0;
pub const ENVIRONMENT_Z: f32 = 2.0;
pub const LOG_Z: f32 = 3.0;
pub const PLAYER_Z: f32 = 4.0;
pub const DEBUG_GRID_Z: f32 = 5.0; // Z-index for debug grid visualization

// Grid dimensions
pub const GRID_SIZE_X: u32 = 15;
pub const GRID_SIZE_Y: u32 = 15;
pub const TILE_SIZE: f32 = 32.0;

// Screen boundaries
pub const SCREEN_BOTTOM: f32 = -320.0;
//pub const SCREEN_TOP: f32 = 250.0;

#[derive(Component)]
/*pub struct WorldGrid;

#[derive(Component)]
pub struct GridTile {
    pub x: u32,
    pub y: u32,
    pub occupied: bool,
}*/

// Add a new resource to control grid visibility
#[derive(Resource)]
pub struct GridDebugConfig {
    pub show_grid: bool,
}

#[derive(Resource)]
pub struct GridConfig {
    pub tile_size: f32,
    pub grid_width: u32,
    pub grid_height: u32,
    pub scroll_speed: f32,
    pub distance_moved: f32,
    pub spawn_threshold: f32,
}

#[derive(Component)]
pub struct GridObject;

// Add a component to mark debug grid entities
#[derive(Component)]
pub struct DebugGridMarker;

// Add a component specifically for grid lines
#[derive(Component)]
pub struct GridLineMarker;

#[derive(Resource)]
pub struct ScrollSpeedTimer(pub Timer);

pub fn setup_world_grid(mut commands: Commands) {
    // Initialize grid configuration
    commands.insert_resource(GridConfig {
        tile_size: TILE_SIZE,
        grid_width: GRID_SIZE_X,
        grid_height: GRID_SIZE_Y,
        scroll_speed: 35.0,
        distance_moved: 0.0,
        spawn_threshold: TILE_SIZE * 10.0,
    });
    
    // Initialize debug grid configuration (ON by default)
    commands.insert_resource(GridDebugConfig {
        show_grid: true,
    });
}



// Centralized grid movement system
pub fn move_grid_objects(
    time: Res<Time>,
    mut grid_config: ResMut<GridConfig>,
    mut query: Query<&mut Transform, With<GridObject>>,
) {
    // Begræns delta_time for at undgå store spikes
    let delta_time = time.delta_secs().min(1.0 / 60.0);

    let movement = grid_config.scroll_speed * delta_time;
    grid_config.distance_moved += movement;

    for mut transform in query.iter_mut() {
        transform.translation.y -= movement;

        // Snap to pixel grid for crisp rendering
        transform.translation.x = transform.translation.x.floor();
        transform.translation.y = transform.translation.y.floor();
    }
}



// Add this system to clean up offscreen objects and spawn new ones
pub fn check_offscreen_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &Transform, &GridObject)>,
    mut grid_config: ResMut<GridConfig>,
) {
    let mut should_spawn_new = false;

    // Check if objects have moved off screen
    for (entity, transform, _) in query.iter() {
        if transform.translation.y < SCREEN_BOTTOM {
            commands.entity(entity).despawn();
            should_spawn_new = true;
        }
    }

    // Update distance counter
    if should_spawn_new {
        grid_config.distance_moved += TILE_SIZE;

        // Check if we've moved enough to spawn new objects
        if grid_config.distance_moved >= grid_config.spawn_threshold {
            // Reset counter
            grid_config.distance_moved = 0.0;

            // This is where you would add logic to spawn new objects
            // For now, we'll just log that we should spawn something
            println!("Should spawn new objects at the top of the screen");
        }
    }
}

// Modified system to toggle grid visibility with G key
pub fn toggle_grid_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grid_debug: ResMut<GridDebugConfig>,
    mut commands: Commands,
    debug_grid_query: Query<Entity, With<DebugGridMarker>>,
    grid_line_query: Query<Entity, With<GridLineMarker>>,
    grid_config: Res<GridConfig>,
) {
    // Toggle grid visibility when G key is pressed
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        // First, always clean up existing debug grid entities
        for entity in debug_grid_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Clean up existing grid lines
        for entity in grid_line_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Toggle the flag
        grid_debug.show_grid = !grid_debug.show_grid;
        
        println!("Grid object visualization: {}", if grid_debug.show_grid { "ON" } else { "OFF" });
    }
}

// Separate system to handle grid visualization based on the boolean flag
pub fn update_debug_grid(
    grid_debug: Res<GridDebugConfig>,
    mut commands: Commands,
    debug_grid_query: Query<Entity, With<DebugGridMarker>>,
    grid_line_query: Query<Entity, With<GridLineMarker>>,
    grid_objects_query: Query<&Transform, (With<GridObject>, Without<DebugGridMarker>, Without<GridLineMarker>)>,
    grid_config: Res<GridConfig>,
) {
    // Check if the grid visualization state has changed
    if grid_debug.show_grid && debug_grid_query.is_empty() && grid_line_query.is_empty() {
        // Grid should be shown but isn't currently visible - create it
        
        // Create grid visualization for each object
        for transform in grid_objects_query.iter() {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 0.5, 0.0, 0.3), // Orange highlight
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        DEBUG_GRID_Z,
                    ),
                    ..default()
                },
                DebugGridMarker,
                GridObject,
            ));
        }
        
        // Draw grid lines
        draw_grid_lines(&mut commands, &grid_config);
    } else if !grid_debug.show_grid && (!debug_grid_query.is_empty() || !grid_line_query.is_empty()) {
        // Grid should be hidden but is currently visible - remove it
        for entity in debug_grid_query.iter() {
            commands.entity(entity).despawn();
        }
        
        for entity in grid_line_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn increase_scroll_speed(
    time: Res<Time>,
    mut score_timer: ResMut<ScrollSpeedTimer>,
    mut grid_config: ResMut<GridConfig>
) {
    score_timer.0.tick(time.delta());

    if score_timer.0.just_finished() {
        grid_config.scroll_speed += 10.;
        println!("Scroll speed: {}", grid_config.scroll_speed);
    }
}

// New helper function to update grid visualization
fn update_grid_visualization(
    commands: &mut Commands,
    grid_objects_query: &Query<&Transform, With<GridObject>>,
    grid_config: &GridConfig,
) {
    // For each grid object, create a visual indicator
    for transform in grid_objects_query.iter() {
        // Create a highlight around each grid object
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 0.5, 0.0, 0.3), // Orange highlight
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    DEBUG_GRID_Z, // Above everything else
                ),
                ..default()
            },
            DebugGridMarker,
            GridObject, // Make it move with the grid
        ));
    }
    
    // Draw grid lines
    draw_grid_lines(commands, grid_config);
}

// New function to draw grid lines
fn draw_grid_lines(commands: &mut Commands, grid_config: &GridConfig) {
    let window_width = grid_config.grid_width as f32 * grid_config.tile_size;
    let window_height = grid_config.grid_height as f32 * grid_config.tile_size;
    
    // Calculate the starting position (top-left corner of the grid)
    let start_x = -(window_width / 2.0);
    let start_y = window_height / 2.0;
    
    // Draw vertical lines
    for i in 0..=grid_config.grid_width {
        let x_pos = start_x + (i as f32 * grid_config.tile_size);
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 1.0, 1.0, 0.5), // Cyan lines
                    custom_size: Some(Vec2::new(1.0, window_height)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    x_pos,
                    start_y - window_height / 2.0, // Center vertically
                    DEBUG_GRID_Z,
                ),
                ..default()
            },
            GridLineMarker,
            GridObject, // Make it move with the grid
        ));
    }
    
    // Draw horizontal lines
    for i in 0..=grid_config.grid_height {
        let y_pos = start_y - (i as f32 * grid_config.tile_size);
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 1.0, 1.0, 0.5), // Cyan lines
                    custom_size: Some(Vec2::new(window_width, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    start_x + window_width / 2.0, // Center horizontally
                    y_pos,
                    DEBUG_GRID_Z,
                ),
                ..default()
            },
            GridLineMarker,
            GridObject, // Make it move with the grid
        ));
    }
}

