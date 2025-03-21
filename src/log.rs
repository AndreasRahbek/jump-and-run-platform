use bevy::prelude::*;
//use crate::collision::Collider;
use super::movement::Movable;

#[derive(Component)]
pub struct Log;

pub fn spawn_log(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<crate::SpawnTimer>
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Log,
            Sprite {
                image: asset_server.load("tileset/log.png"),
                ..default()
            },
            Transform::from_xyz(0.0, 100.0, 2.0),
            Movable { speed: 50.0 },
            //Collider,

        ));
    }
}