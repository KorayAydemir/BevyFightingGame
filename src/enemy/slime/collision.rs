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
    q_player_transform: Query<&Transform, With<Player>>,
    q_colliders: Query<&Parent, With<Collider>>,
    mut ev_collision_events: EventReader<CollisionEvent>
) {
    let player_translation = q_player_transform.single().translation;

    for ev in ev_collision_events.read() {
        println!("in loop " );
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_,_,_ ) => continue,
        };
        println!("target {:?}", target);

        match q_colliders.get(*target) {
            Ok(parent) => println!("hit {:?}", parent),
            Err(_) => continue
        }

        //let enemy = if &player.collid

    }

    for transform in &q_slime_transforms {
        let slime_translation = transform.translation;

        let distance = slime_translation.distance(player_translation);

        if distance < 10. {
            //println!("Player hit by slime!");
        }
    }
}
