use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    state::SlimeState, Slime, MAX_RANDOM_SLIME_SCALE, MEGA_SLIME_SCALE, MIN_RANDOM_SLIME_SCALE,
};
use crate::{common::sprite::AnimationTimer, enemy::Enemy, player::Player};
use rand::Rng;

pub struct SlimeSpawnPlugin;
impl Plugin for SlimeSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SlimeSpawnEvent>()
            .insert_resource(SlimeSpawnTimer(Timer::from_seconds(
                2.0,
                TimerMode::Repeating,
            )))
            .add_systems(Update, timer_system)
            .add_systems(Update, spawn_slimes.run_if(on_event::<SlimeSpawnEvent>()));
    }
}

#[derive(Resource)]
struct SlimeSpawnTimer(Timer);

#[derive(Event)]
struct SlimeSpawnEvent {
    amount: usize,
    scale: Vec3,
}

fn random_slime_size() -> Vec3 {
    let mut rng = rand::thread_rng();

    if rng.gen_range(0.0..100.0) < 5. {
        Vec3::splat(MEGA_SLIME_SCALE)
    } else {
        Vec3::splat(rng.gen_range(MIN_RANDOM_SLIME_SCALE..MAX_RANDOM_SLIME_SCALE))
    }
}

fn timer_system(
    time: Res<Time>,
    mut slime_spawn_timer: ResMut<SlimeSpawnTimer>,
    mut spawn_slime_event: EventWriter<SlimeSpawnEvent>,
) {
    slime_spawn_timer.0.tick(time.delta());

    let mut rng = rand::thread_rng();
    let random_slime_amount = rng.gen_range(5..15);
    if slime_spawn_timer.0.finished() {
        spawn_slime_event.send(SlimeSpawnEvent {
            amount: random_slime_amount,
            scale: random_slime_size(),
        });
    }
}

fn random_point_around_player(player_pos: Vec3, min_distance: f32, max_distance: f32) -> Vec3 {
    let mut rng = rand::thread_rng();

    // Generate a random angle in radians
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

    // Generate a random distance within the specified range
    let distance = rng.gen_range(min_distance..max_distance);

    let new_x = player_pos.x + angle.cos() * distance;
    let new_y = player_pos.y + angle.sin() * distance;

    Vec3 {
        x: new_x,
        y: new_y,
        z: 0.,
    }
}

fn spawn_slimes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut slime_spawn_event: EventReader<SlimeSpawnEvent>,
    q_player: Query<&Transform, With<Player>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 24), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut slime_amount = 0;
    let mut slime_scale = Vec3::new(0., 0., 0.);
    for ev in slime_spawn_event.read() {
        slime_amount = ev.amount;
        slime_scale = ev.scale;
    }

    let player_pos = match q_player.get_single() {
        Ok(player_transform) => player_transform.translation,
        Err(_) => return,
    };

    for i in 0..=slime_amount {
        let slime_pos = random_point_around_player(player_pos, 300.0, 600.0);
        let slime_spawn_pos = Vec3::new(slime_pos.x, slime_pos.y, 0.);

        let slime = commands
            .spawn((
                //RigidBody::KinematicPositionBased,
                Name::new(format!("Slime {i}")),
                RigidBody::Dynamic,
                SpriteSheetBundle {
                    transform: Transform::from_translation(slime_spawn_pos).with_scale(slime_scale),
                    texture: asset_server.load("textures/mobs/slime-blue.png"),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 0,
                    },
                    ..default()
                },
                Slime::new(i, slime_scale.x.round() as usize),
                SlimeState::Patrolling,
                Enemy { damage: 0.5 },
                AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            ))
            .id();

        let collider = commands
            .spawn((
                Collider::ball(6.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0., -5., 0.,
                ))),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            ))
            .id();

        commands.entity(slime).push_children(&[collider]);
    }
}
