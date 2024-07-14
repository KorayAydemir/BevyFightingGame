use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemy::Enemy,
    player::{GotHitInfo, Player},
};

use super::{PlayerEvents, PlayerSet};

pub struct PlayerCollisionPlugin;
impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, slime_collision.in_set(PlayerSet));
    }
}

pub fn slime_collision(
    mut q_player: Query<&Player>,
    q_enemies: Query<&Enemy>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    q_collider_parents: Query<&Parent, With<Collider>>,
    mut player_events: EventWriter<PlayerEvents>,
) {
    let player = q_player.single_mut();

    for ev in ev_collision_events.read() {
        let (collider_entity1, collider_entity2) = match ev {
            CollisionEvent::Started(collider1, collider2, _) => (collider1, collider2),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let enemy_parent = if &player.collider_entity == collider_entity1 {
            let Ok(parent) = q_collider_parents.get(*collider_entity2) else {
                continue;
            };
            parent
        } else if &player.collider_entity == collider_entity2 {
            let Ok(parent) = q_collider_parents.get(*collider_entity1) else {
                continue;
            };
            parent
        } else {
            continue;
        };

        let Ok(enemy) = q_enemies.get(enemy_parent.get()) else {
            return;
        };

        player_events.send(PlayerEvents::GotHit(GotHitInfo {
            damage: enemy.damage,
        }));
    }
}
