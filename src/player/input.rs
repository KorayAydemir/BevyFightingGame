use bevy::prelude::*;
use bevy_hanabi::{
    AccelModifier, Attribute, ColorOverLifetimeModifier, EffectAsset, ExprWriter, Gradient,
    ParticleEffect, ParticleEffectBundle, SetAttributeModifier, SetPositionCone3dModifier,
    SetVelocitySphereModifier, ShapeDimension, SizeOverLifetimeModifier, Spawner,
};
use bevy_math::Vec2;

use super::{
    state::{PlayerChangedState, PlayerState},
    Player,
};

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
}

fn player_movement(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    player_input.move_direction = direction;
}

fn use_fire_spell(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    q_player: Query<(Entity, &Player)>,
    mut player_changed_state: EventReader<PlayerChangedState>,
) {
    for changed_state in player_changed_state.read() {
        if changed_state.new_state != PlayerState::CastingSpell {
            return;
        }

        let mut color_gradient = Gradient::new();
        color_gradient.add_key(0.0, Vec4::splat(1.0));
        color_gradient.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
        color_gradient.add_key(0.4, Vec4::new(1.0, 0.0, 0.0, 1.0));
        color_gradient.add_key(1.0, Vec4::splat(0.0));

        let mut size_gradient = Gradient::new();
        size_gradient.add_key(0.0, Vec2::splat(0.1));
        size_gradient.add_key(0.5, Vec2::splat(0.5));
        size_gradient.add_key(0.8, Vec2::splat(0.08));
        size_gradient.add_key(1.0, Vec2::splat(0.0));

        let writer = ExprWriter::new();

        let age1 = writer.lit(0.).expr();
        let init_age1 = SetAttributeModifier::new(Attribute::AGE, age1);

        let lifetime1 = writer.lit(1.5).expr();
        let init_lifetime1 = SetAttributeModifier::new(Attribute::LIFETIME, lifetime1);

        // Add constant downward acceleration to simulate gravity
        let accel1 = writer.lit(Vec3::Y * -3.).expr();
        let update_accel1 = AccelModifier::new(accel1);

        let init_pos = SetPositionCone3dModifier {
            base_radius: writer.lit(0.).expr(),
            top_radius: writer.lit(10.).expr(),
            height: writer.lit(20.).expr(),
            dimension: ShapeDimension::Volume,
        };

        let init_vel = SetVelocitySphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            speed: writer.lit(10.).expr(),
        };

        let effect = effects.add(
            EffectAsset::new(vec![32768], Spawner::rate(500.0.into()), writer.finish())
                .with_name("emit:rate")
                .init(init_pos)
                // Make spawned particles move away from the emitter origin
                .init(init_vel)
                .init(init_age1)
                .init(init_lifetime1)
                .update(update_accel1)
                .render(ColorOverLifetimeModifier {
                    gradient: color_gradient,
                })
                .render(SizeOverLifetimeModifier {
                    gradient: size_gradient,
                    screen_space_size: false,
                }),
        );

        let fire_effect = commands
            .spawn((
                Name::new("emit:rate"),
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect),
                    transform: Transform::from_translation(Vec3::new(30., 0., 0.))
                        .with_rotation(Quat::from_rotation_z(1.)),
                    ..Default::default()
                },
            ))
            .id();

        let (player_id, _p) = q_player.get_single().unwrap();
        commands.entity(player_id).push_children(&[fire_effect]);
    }
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(Update, player_movement)
            .add_systems(Update, use_fire_spell);
    }
}
