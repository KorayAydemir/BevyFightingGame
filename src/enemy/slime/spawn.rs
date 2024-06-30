use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{state::SlimeState, Slime, MAX_SLIME_SCALE, MIN_SLIME_SCALE};
use crate::{common::sprite::AnimationTimer, enemy::Enemy};
use rand::Rng;

pub struct SlimeSpawnPlugin;
impl Plugin for SlimeSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_slimes::<2>);
    }
}

fn spawn_slimes<const SLIME_AMOUNT: usize>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 24.), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for i in 0..=SLIME_AMOUNT {
        let mut rng = rand::thread_rng();
        let spawn_pos_x = rng.gen_range(-100.0..500.0);
        let spawn_pos_y = rng.gen_range(-100.0..400.0);
        let slime_spawn_pos = Vec3::new(spawn_pos_x, spawn_pos_y, 0.);

        let random_slime_scale = rng.gen_range(MIN_SLIME_SCALE..MAX_SLIME_SCALE);
        let slime_scale = Vec3::splat(random_slime_scale);

        let slime = commands.spawn((
            //RigidBody::KinematicPositionBased,
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
            Slime::new(i),
            SlimeState::Patrolling,
            Enemy { damage: 0.5 },
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        )).id();

        let collider = commands.spawn((
            Collider::ball(6.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., -5., 0.))),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC
        )).id();

        commands.entity(slime).push_children(&[collider]);
    }
}
