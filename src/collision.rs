use bevy::prelude::*;
use crate::character::Player;

#[derive(Component)]
pub struct Collider{
    pub size: Vec2,
}

pub fn check_collision(
    player_query: Query<(&Collider, Entity, &Transform,  &Player), With<Player>>,
    collider_query: Query<(&Collider, &Transform), (With<Collider>, Without<Player>)>,
    mut commands: Commands,
) {
    for(player_collider, player_entity, player_transform, player) in player_query.iter(){
        if player.is_jumping {
            return;
        }
        let player_size = player_collider.size;
        let player_pos = player_transform.translation.truncate();

        for(collider, collider_transform) in collider_query.iter(){
            let collider_size = collider.size;
            let collider_pos = collider_transform.translation.truncate();

            // AABB collision check
            if player_pos.x - player_size.x / 2.0 < collider_pos.x + collider_size.x / 2.0
                && player_pos.x + player_size.x / 2.0 > collider_pos.x - collider_size.x / 2.0
                && player_pos.y - player_size.y / 2.0 < collider_pos.y + collider_size.y / 2.0
                && player_pos.y + player_size.y / 2.0 > collider_pos.y - collider_size.y / 2.0
            {
                commands.entity(player_entity).despawn();
            }
        }
    }
}




