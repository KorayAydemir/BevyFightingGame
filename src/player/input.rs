use std::any::Any;

use crate::player::spawn::Velocity;
use bevy::prelude::*;
use bevy_hanabi::{
    AccelModifier, Attribute, ColorOverLifetimeModifier, EffectAsset, Gradient, Module,
    ParticleEffect, ParticleEffectBundle, SetAttributeModifier, SetPositionSphereModifier,
    SetVelocitySphereModifier, ShapeDimension, Spawner,
};

use super::Player;

fn player_movement(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut velocity = query.single_mut();

    velocity.x = 0.;
    velocity.y = 0.;

    if keys.pressed(KeyCode::KeyD) {
        velocity.x = 1.
    }

    if keys.pressed(KeyCode::KeyA) {
        velocity.x = -1.
    }

    if keys.pressed(KeyCode::KeyW) {
        velocity.y = 1.
    }

    if keys.pressed(KeyCode::KeyS) {
        velocity.y = -1.
    }
}

fn use_fire_spell(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut q_player: Query<(Entity, &Player)>
) {
    if keys.pressed(KeyCode::KeyC) {
        let mut gradient = Gradient::new();
        gradient.add_key(0., Vec4::new(1., 0., 0., 1.));
        gradient.add_key(1., Vec4::splat(0.));

        let mut module = Module::default();

        let init_pos = SetPositionSphereModifier {
            center: module.lit(Vec3::ZERO),
            radius: module.lit(0.05),
            dimension: ShapeDimension::Surface,
        };

        let init_vel = SetVelocitySphereModifier {
            center: module.lit(Vec3::ZERO),
            speed: module.lit(6.),
        };

        let lifetime = module.lit(10.);
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let accel = module.lit(Vec3::new(0., -3., 0.));
        let update_accel = AccelModifier::new(accel);

        let effect = EffectAsset::new(vec![32768], Spawner::rate(5.0.into()), module)
            .with_name("flame_effect")
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .update(update_accel)
            .render(ColorOverLifetimeModifier { gradient });

        let effect_handle = effects.add(effect);

        let fire_particles = commands.spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(Vec3::Y),
            ..default()
        }).id();

        let (player_id, _p) = q_player.get_single().unwrap();

        commands.entity(player_id).push_children(&[fire_particles]);
    }
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .add_systems(Update, use_fire_spell);
    }
}
