use bevy::asset::{AssetServer, Assets};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::*;
use crate::collision::Collider;
use crate::world_grid::{PLAYER_Z};

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

const PLAYER_HITBOX_SIZE: Vec2 = Vec2::new(5., 1.);
const ANIMATION_SPEED: f32 = 0.1;


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
    let texture = asset_server.load("textures/character/human.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 1, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices {first: 0, last: 5};
    // Update the Collider initialization to include is_trigger
    commands.spawn((
        Player::default(),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform {
            translation: Vec3::new(0., 0., PLAYER_Z),
            ..Default::default() // Beholder rotation som identity
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        Collider{
            size: PLAYER_HITBOX_SIZE,
            //is_trigger: false, // Add this field
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
    mut player: Query<(&mut Collider, &mut Player), With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut collider, mut player) in &mut player.iter_mut() {
        if kb_input.pressed(KeyCode::Space) && !player.is_jumping {
            player.is_jumping = true;
            println!("Jumping");
            jump_timer.0.reset();
        }

        jump_timer.0.tick(time.delta());

        if jump_timer.0.just_finished() {
            println!("Done Jumping");
            player.is_jumping = false;
        }
    }

}



