use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;

use super::Slime;

pub struct SlimeCollisionPlugin;
impl Plugin for SlimeCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_collision);
    }
}

fn player_collision(
    q_slime_transforms: Query<&Transform, With<Slime>>,
    q_player: Query<(&Transform, &Player), With<Player>>,
    q_collider_parents: Query<&Parent, With<Collider>>,
    mut ev_collision_events: EventReader<CollisionEvent>
) {
    let (player_transform, player) = q_player.single();

    for ev in ev_collision_events.read() {
        let (collider_entity1, collider_entity2) = match ev {
            CollisionEvent::Started(collider1, collider2, _) => (collider1, collider2),
            CollisionEvent::Stopped(_,_,_ ) => continue,
        };

         let enemy_parent = if &player.collider_entity == collider_entity1 {
            match q_collider_parents.get(*collider_entity1) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else if &player.collider_entity == collider_entity2 {
            match q_collider_parents.get(*collider_entity2) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else {
            continue;
        };
    }

    for transform in &q_slime_transforms {
        let slime_translation = transform.translation;

        let distance = slime_translation.distance(player_transform.translation);

        if distance < 10. {
            //println!("Player hit by slime!");
        }
    }
}
