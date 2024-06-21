use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{enemy::Enemy, player::{Health, Player}};

pub struct PlayerCollisionPlugin;
impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, slime_collision);
    }
}

pub fn slime_collision(
    mut q_player: Query<(&Player, &Transform, &mut Health), With<Player>>,
    q_enemies: Query<(&Transform, &Enemy)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    q_collider_parents: Query<&Parent, With<Collider>>,
) {
    let (player, transform, mut health) = q_player.single_mut();


    for ev in ev_collision_events.read() {
        let (collider_entity1, collider_entity2) = match ev {
            CollisionEvent::Started(collider1, collider2, _) => (collider1, collider2),
            CollisionEvent::Stopped(_,_,_ ) => continue,
        };

         let enemy_parent = if &player.collider_entity == collider_entity1 {
            match q_collider_parents.get(*collider_entity2) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else if &player.collider_entity == collider_entity2 {
            match q_collider_parents.get(*collider_entity1) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else {
            println!("continue");
            continue;
        };

        let (enemy_transform, enemy) = q_enemies.get(enemy_parent.get()).unwrap();

        println!("reduce hp");
        health.health -= enemy.damage;
    }


}
