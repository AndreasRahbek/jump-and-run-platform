use bevy::prelude::*;

#[derive(Component)]
pub struct Movable {
    pub speed: f32,  // Use this value directly for all movement calculations
}

pub fn move_entities(
    mut query: Query<(&mut Transform, &Movable)>,
    time: Res<Time>,
) {
    for (mut transform, movable) in query.iter_mut() {
        // Apply consistent movement using the speed value from Movable
        transform.translation.y -= movable.speed * time.delta_secs();
    }
}
