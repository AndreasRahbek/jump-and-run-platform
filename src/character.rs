use bevy::asset::{AssetServer, Assets};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::*;
use crate::collision::Collider;
use crate::microbit::JumpSignal;
use crate::world_grid::{PLAYER_Z};
use std::time::Duration;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct JumpTimer(pub Timer);


#[derive(Component, Default)]
pub struct Player {
    pub is_jumping: bool,
}

const PLAYER_HITBOX_SIZE: Vec2 = Vec2::new(8., 1.);
const ANIMATION_SPEED: f32 = 0.1;

#[derive(Component)]
pub struct AnimationConfig {
    pub run_texture: Handle<Image>,
    pub run_layout: Handle<TextureAtlasLayout>,
    pub run_indices: (usize, usize),

    pub jump_texture: Handle<Image>,
    pub jump_layout: Handle<TextureAtlasLayout>,
    pub jump_indices: (usize, usize),

    pub fps: u8,
    pub frame_timer: Timer,
    pub state: AnimationState,
}





#[derive(Component, PartialEq, Eq)]
pub enum AnimationState {
    Run,
    Jump,
}


impl AnimationConfig {
    pub fn new(
        run_texture: Handle<Image>,
        run_layout: Handle<TextureAtlasLayout>,
        run_indices: (usize, usize),
        jump_texture: Handle<Image>,
        jump_layout: Handle<TextureAtlasLayout>,
        jump_indices: (usize, usize),
        fps: u8,
    ) -> Self {
        Self {
            run_texture,
            run_layout,
            run_indices,
            jump_texture,
            jump_layout,
            jump_indices,
            fps,
            frame_timer: timer_from_fps(fps),
            state: AnimationState::Run,
        }
    }
}


pub fn timer_from_fps(fps: u8) -> Timer {
    Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
}


pub fn animate_sprite(time: Res<Time>, mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for(indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                }
            }
        }
    }
}

pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let run_texture = asset_server.load("textures/character/run.png");
    let jump_texture = asset_server.load("textures/character/jump.png");

    let run_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 1, None, None));
    let jump_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 1, None, None));

    let animation_config = AnimationConfig::new(
        run_texture.clone(),
        run_layout.clone(),
        (0, 5),
        jump_texture.clone(),
        jump_layout.clone(),
        (0, 2),
        10,
    );


    commands.spawn((
        Player::default(),
        Sprite {
            image: run_texture,
            texture_atlas: Some(TextureAtlas {
                layout: run_layout,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0., 0., PLAYER_Z),
        animation_config,
        Collider {
            size: PLAYER_HITBOX_SIZE,
        }
    ));
}


const PLAYER_SPEED: f32 = 50.0;

pub fn move_character_horizontal(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    // Check if inputs are detected
    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    // Progressively update the player's position over time
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();

    // Apply movement
    player.translation.x += move_delta.x;

    // Limit player movement to screen bounds
    let screen_bound = 220.0;
    player.translation.x = player.translation.x.clamp(-screen_bound, screen_bound);
}

pub fn jump(
    time: Res<Time>,
    mut jump_timer: ResMut<JumpTimer>,
    mut player_query: Query<(&mut Player, &mut AnimationConfig, &mut Sprite), With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    signal: Res<JumpSignal>,
) {
    let mut microbit_triggered = false;

    if let Ok(mut lock) = signal.0.lock() {
        if *lock {
            microbit_triggered = true;
            *lock = false;
        }
    }

    for (mut player, mut animation, mut sprite) in &mut player_query {
        if (kb_input.pressed(KeyCode::Space) || microbit_triggered) && !player.is_jumping {
            player.is_jumping = true;
            animation.state = AnimationState::Jump;

            // Skift til jump sprite-sheet
            sprite.image = animation.jump_texture.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: animation.jump_layout.clone(),
                index: animation.jump_indices.0,
            });

            jump_timer.0.reset();
        }

        jump_timer.0.tick(time.delta());

        if jump_timer.0.just_finished() {
            player.is_jumping = false;
            animation.state = AnimationState::Run;

            // Skift tilbage til run sprite-sheet
            sprite.image = animation.run_texture.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: animation.run_layout.clone(),
                index: animation.run_indices.0,
            });
        }
    }
}


pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            let (first, last) = match config.state {
                AnimationState::Run => config.run_indices,
                AnimationState::Jump => config.jump_indices,
            };

            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= last {
                    atlas.index = first;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}





