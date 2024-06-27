use crate::{
    enemy::Enemy, player::{spawn::spawn_player, Health, Player, PLAYER_MAX_HEALTH}, GameState
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct HealthUiPlugin;
impl Plugin for HealthUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, update_hearts.run_if(in_state(GameState::Playing)).after(spawn_player));
        app.add_systems(Update, update_hearts.run_if(in_state(GameState::Playing)).run_if(run_if_player_got_hit));
    }
}

fn run_if_player_got_hit(
    q_player: Query<(&Player, &Transform), With<Player>>,
    q_enemies: Query<(&Transform, &Enemy)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    q_collider_parents: Query<&Parent, (With<Collider>, Without<Enemy>, Without<Player>)>,
) -> bool {
    let (player, transform) = q_player.single();

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

        //let (enemy_transform, enemy) = q_enemies.get(enemy_parent.get()).unwrap();

        return true;
    }

    false
}

#[derive(Component)]
struct UiHearts;

fn get_heart_image_for_hp(
    heart_index: f32,
    health: f32,
    asset_server: &Res<AssetServer>,
) -> Handle<Image> {
    let heart_img = if (health + 0.5) == heart_index {
        "heart_half.png"
    } else if heart_index <= health {
        "heart_full.png"
    } else {
        "heart_empty.png"
    };

    asset_server.load(heart_img)
}

fn update_hearts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_player_health: Query<&Health, With<Player>>,
    ui_hearts: Query<Entity, With<UiHearts>>,
) {
    let health = q_player_health.single().health;
    println!("update hearts with hp: {health}");

    if let Ok(ui_hearts) = ui_hearts.get_single() {
        commands.entity(ui_hearts).despawn_recursive();
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(10.0),
                    height: Val::Percent(10.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            UiHearts,
        ))
        .with_children(|parent| {
            for i in 1..=PLAYER_MAX_HEALTH as i32 {
                parent.spawn(ImageBundle {
                    image: UiImage::new(get_heart_image_for_hp(i as f32, health, &asset_server)),
                    ..default()
                });
            }
        });
}
