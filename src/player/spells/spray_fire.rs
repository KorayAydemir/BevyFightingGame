use bevy::prelude::*;
use std::time::Duration;

use bevy_rapier2d::prelude::*;

use bevy_hanabi::prelude::*;

use super::state::PlayerState;

pub fn create_fire_cone_effect(effects: &mut ResMut<Assets<EffectAsset>>) -> Handle<EffectAsset> {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::splat(1.0));
    color_gradient.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
    color_gradient.add_key(0.2, Vec4::new(1.0, 0.0, 0.0, 1.0));
    color_gradient.add_key(1.0, Vec4::splat(0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.2, Vec2::splat(2.5));
    size_gradient.add_key(0.4, Vec2::splat(2.0));
    size_gradient.add_key(0.7, Vec2::splat(1.0));
    size_gradient.add_key(1.0, Vec2::splat(0.5));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(1.8).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -3.).expr();
    let update_accel = AccelModifier::new(accel);

    let init_pos = SetPositionCone3dModifier {
        base_radius: writer.lit(0.).expr(),
        top_radius: writer.lit(50.).expr(),
        height: writer.lit(50.).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(10.).expr(),
    };

    effects.add(
        EffectAsset::new(
            vec![32768],
            Spawner::once(1000.0.into(), true),
            writer.finish(),
        )
        .with_name("spell_spray_fire")
        .init(init_pos)
        // Make spawned particles move away from the emitter origin
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        }),
    )
}

fn create_fire_cone_hitbox(
    commands: &mut Commands,
    player_entity: Entity,
    player_sprite: &Sprite,
    despawn_after_secs: f32,
    player_transform: &Transform,
) {
    let hitbox_transform = Transform::from_translation(Vec3::new(player_transform.translation.x - 40. , player_transform.translation.y, 0.));
    commands
        .spawn((
            FireConeHitbox(Timer::from_seconds(despawn_after_secs, TimerMode::Once)),
            Collider::cuboid(40., 45.),
            TransformBundle::from_transform(hitbox_transform),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        ));
}
