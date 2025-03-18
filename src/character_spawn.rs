use bevy::asset::{AssetServer, Assets};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

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
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform {
            translation: Vec3::new(0., 0., 2.),
            scale: Vec3::splat(1.0),
            ..Default::default() // Beholder rotation som identity
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

