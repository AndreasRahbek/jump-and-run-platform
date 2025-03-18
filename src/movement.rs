use bevy::prelude::*;

#[derive(Component)]
pub struct Movable {
    pub speed: f32,
}

pub fn move_entities(
    mut query: Query<(&mut Transform, &Movable)>,
    time: Res<Time>,
) {
    for (mut transform, movable) in query.iter_mut() {
        transform.translation.y -= movable.speed * time.delta_secs();
    }
}
