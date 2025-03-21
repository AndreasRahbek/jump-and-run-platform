use bevy::math::Vec3;
use bevy::prelude::{Camera2d, Commands, Transform, Msaa};

pub fn setup_camera(mut commands: Commands){
    commands.spawn((
        Camera2d::default(),
        Transform::from_scale(Vec3::splat(0.1)),
        Msaa::Off,
    ));
}
