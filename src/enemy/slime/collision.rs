use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::{spells::PlayerMeleeHitbox, Player}, GameState};

use super::Slime;

pub struct SlimeCollisionPlugin;
impl Plugin for SlimeCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_collision.run_if(in_state(GameState::Playing)))
            .add_systems(Update, player_melee_collisions.run_if(in_state(GameState::Playing)));
    }
}

fn player_collision(
    q_slime_transforms: Query<&Transform, With<Slime>>,
    q_player: Query<(&Transform, &Player), With<Player>>,
    q_collider_parents: Query<&Parent, With<Collider>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let (player_transform, player) = q_player.single();

    for ev in ev_collision_events.read() {
        let (collider_entity1, collider_entity2) = match ev {
            CollisionEvent::Started(collider1, collider2, _) => (collider1, collider2),
            CollisionEvent::Stopped(_, _, _) => continue,
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

fn player_melee_collisions(
    mut q_melee_hitbox: Query<Entity, With<PlayerMeleeHitbox>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    mut q_collider_parents: Query<&Parent, (With<Collider>, Without<Player>)>,
    mut commands: Commands,
) {
    for ev in ev_collision_events.read() {
        let (collider_entity1, collider_entity2) = match ev {
            CollisionEvent::Started(collider1, collider2, _) => (collider1, collider2),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let melee_hitbox = match q_melee_hitbox.get_single() {
            Ok(hitbox) => hitbox,
            Err(_) => continue,
        };

        let enemy_parent = if &melee_hitbox == collider_entity1 {
            match q_collider_parents.get(*collider_entity2) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else if &melee_hitbox == collider_entity2 {
            match q_collider_parents.get(*collider_entity1) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else {
            continue;
        };

        commands.entity(**enemy_parent).despawn_recursive();
    }
}
