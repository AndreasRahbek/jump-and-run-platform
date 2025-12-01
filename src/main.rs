mod character;
mod background;
mod moving_road;
mod log;
mod collision;
mod environment;
mod world_grid;
mod microbit;
mod scoreboard;

use bevy::prelude::*;
use character::*;
use background::*;
use moving_road::*;
use collision::*;
use log::*;
use environment::*;
use world_grid::*;
use crate::microbit::{setup_serial_listener, JumpSignal};
use crate::scoreboard::{
    increase_score_system, scoreboard_system, setup_scoreboard, ScoreText, ScoreTimer,
    update_death_scoreboard_ui, handle_input_system, update_name_input_display,
    ScoreboardState, GameState
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .init_state::<GameState>()
        .insert_resource(JumpSignal::default())
        .insert_resource(<ScoreText>::default())
        .insert_resource(ScoreTimer(Timer::from_seconds(1.0 / 3.0, TimerMode::Repeating)))
        .insert_resource(ScrollSpeedTimer(Timer::from_seconds(10., TimerMode::Repeating)))
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(JumpTimer(Timer::from_seconds(0.2, TimerMode::Once)))
        
        // One-time setup
        .add_systems(Startup, (
            setup_serial_listener,
            setup_pixel_grid,
        ))

        // Gameplay setup (runs on restart)
        .add_systems(OnEnter(GameState::Playing), (
            setup_scoreboard,
            setup_world_grid,
            setup_grid,
            setup_road,
            setup_environment,
            setup_character,
        ).chain())

        // Cleanup when game over ends (restarting)
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        
        .add_systems(Update, (
            execute_animations,
            handle_player_death,
            increase_score_system,
            scoreboard_system,
            update_road,
            update_environment,
            increase_scroll_speed,
            remove_old_road,
            remove_old_environment,
            update_background,
            move_character_horizontal,
            remove_old_background,
            animate_sprite,
            jump,
            move_grid_objects,
            check_collision,
            spawn_log,
            check_offscreen_objects,
        ).run_if(in_state(GameState::Playing)))

        .add_systems(Update, (
            update_death_scoreboard_ui,
            handle_input_system,
            update_name_input_display,
        ).run_if(in_state(GameState::GameOver)))

        // Always run
        .add_systems(Update, (
            background::fit_canvas,
        ))
        
        .run();
}

fn cleanup_game_over(
    mut commands: Commands,
    query: Query<Entity, Or<(
        With<Player>, 
        With<MovingRoad>, 
        With<EnvironmentObject>, 
        With<Log>, 
        With<TileGrid>,
        With<crate::scoreboard::DeathScoreDisplay>
    )>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
