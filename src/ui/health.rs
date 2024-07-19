use crate::player::{spawn::spawn_player, Health, Player, PlayerEvents, PLAYER_MAX_HEALTH};
use bevy::prelude::*;

pub struct HealthUiPlugin;
impl Plugin for HealthUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, update_hearts.after(spawn_player));
        app.add_systems(Update, update_hearts);
    }
}

#[derive(Component)]
struct UiHearts;

fn update_hearts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_player_health: Query<&Health, (With<Player>, Changed<Health>)>,
    ui_hearts: Query<Entity, With<UiHearts>>,
) {
    let player_health = match q_player_health.get_single() {
        Ok(player_health) => player_health.health,
        Err(_) => return,
    };

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
            for heart_img in get_hearts_images(player_health) {
                parent.spawn(ImageBundle {
                    image: UiImage::new(asset_server.load(heart_img)),
                    ..default()
                });
            }
        });
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn get_hearts_images(health: f32) -> Vec<&'static str> {
    // we need this guard because rest of the logic assumes health is positive
    if health <= 0.0 {
        return vec!["heart_empty.png"; PLAYER_MAX_HEALTH as usize];
    }

    let full_hearts = vec!["heart_full.png"; health.floor() as usize];

    let has_half_heart = health - health.floor() >= 0.5;
    let maybe_half_heart = if has_half_heart {
        vec!["heart_half.png"]
    } else {
        vec![]
    };

    let empty_hearts = vec![
        "heart_empty.png";
        (PLAYER_MAX_HEALTH - health.floor() - if has_half_heart { 1.0 } else { 0.0 })
            as usize
    ];

    [full_hearts, maybe_half_heart, empty_hearts].concat()
}
