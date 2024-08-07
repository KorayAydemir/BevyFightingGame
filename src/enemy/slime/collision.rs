use bevy::prelude::*;
use crate::player::spells::FireConeHitbox;
use bevy_rapier2d::prelude::*;

use crate::player::{spells::PlayerMeleeHitbox, Player, PlayerEvents};

use super::Slime;

pub struct SlimeCollisionPlugin;
impl Plugin for SlimeCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_collision)
            .add_systems(Update, (player_melee_hitbox_collisions, player_fire_cone_collisions));
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

fn player_melee_hitbox_collisions(
    q_melee_hitbox: Query<Entity, With<PlayerMeleeHitbox>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    q_collider_parents: Query<&Parent, (With<Collider>, Without<Player>)>,
    q_slime: Query<&Slime>,
    mut ev_player_events: EventWriter<PlayerEvents>,
    mut commands: Commands,
) {
    let Ok(melee_hitbox) = q_melee_hitbox.get_single() else {
        return
    };

    for ev in ev_collision_events.read() {
        let (collider_entity1, collider_entity2) = match ev {
            CollisionEvent::Started(collider1, collider2, _) => (collider1, collider2),
            CollisionEvent::Stopped(_, _, _) => continue,
        };
        let enemy_parent = if &melee_hitbox == collider_entity1 {
            let Ok(parent) = q_collider_parents.get(*collider_entity2) else {
                continue;
            };
            parent
        } else if &melee_hitbox == collider_entity2 {
            let Ok(parent) = q_collider_parents.get(*collider_entity1) else {
                continue;
            };
            parent
        } else {
            continue;
        };

        let slime_points = match q_slime.get(**enemy_parent) {
            Ok(slime) => slime.points,
            Err(_) => continue,
        };
        ev_player_events.send(PlayerEvents::KilledSlime(slime_points));

        commands.entity(**enemy_parent).despawn_recursive();
    }
}

fn player_fire_cone_collisions(
    q_melee_hitbox: Query<Entity, With<FireConeHitbox>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    q_collider_parents: Query<&Parent, (With<Collider>, Without<Player>)>,
    q_slime: Query<&Slime>,
    mut ev_player_events: EventWriter<PlayerEvents>,
    mut commands: Commands,
) {
    let Ok(fire_cone_hitbox) = q_melee_hitbox.get_single() else {
        return
    };

    for ev in ev_collision_events.read() {
        let (collider_entity1, collider_entity2) = match ev {
            CollisionEvent::Started(collider1, collider2, _) => (collider1, collider2),
            CollisionEvent::Stopped(_, _, _) => continue,
        };
        let enemy_parent = if &fire_cone_hitbox == collider_entity1 {
            let Ok(parent) = q_collider_parents.get(*collider_entity2) else {
                continue;
            };
            parent
        } else if &fire_cone_hitbox == collider_entity2 {
            let Ok(parent) = q_collider_parents.get(*collider_entity1) else {
                continue;
            };
            parent
        } else {
            continue;
        };

        let slime_points = match q_slime.get(**enemy_parent) {
            Ok(slime) => slime.points,
            Err(_) => continue,
        };
        ev_player_events.send(PlayerEvents::KilledSlime(slime_points));

        commands.entity(**enemy_parent).despawn_recursive();
    }
}
