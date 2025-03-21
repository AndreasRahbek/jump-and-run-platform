use bevy::prelude::*;
use crate::collision::Collider;
use crate::grid::GAME_LAYERS;
use super::movement::Movable;

#[derive(Component)]
pub struct Log;
#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

const LOG_SIZE: Vec2 = Vec2::new(13.0, 1.);

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
            Transform::from_xyz(0.0, 100.0, 2.0),
            Movable { speed: 50.0 },
            GAME_LAYERS,
            Collider{
                size: LOG_SIZE,
            },
        ));
    }
}