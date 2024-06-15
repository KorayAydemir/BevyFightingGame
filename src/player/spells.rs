use std::time::Duration;
use bevy::{prelude::*, utils::HashMap};

use bevy_hanabi::{
    AccelModifier, Attribute, ColorOverLifetimeModifier, EffectAsset, ExprWriter, Gradient,
    ParticleEffect, ParticleEffectBundle, SetAttributeModifier, SetPositionCone3dModifier,
    SetVelocitySphereModifier, ShapeDimension, SizeOverLifetimeModifier, Spawner,
};

use super::{
    state::PlayerState,
    Player,
};

#[derive(Debug, PartialEq, Clone, Copy, Component, Hash, Eq)]
pub struct SpellDetails {
    pub cast_time: u32,
    pub cooldown: u32,
    pub mana_cost: u32,
}

#[derive(Debug, PartialEq, Clone, Copy, Component, Hash, Eq)]
pub enum Spell {
    SprayFire,
    BlastWave,
}

impl Spell {
    fn details(self) -> SpellDetails {
        match self {
            Spell::SprayFire => SpellDetails {
                cast_time: 1,
                cooldown: 2,
                mana_cost: 10,
            },
            Spell::BlastWave => SpellDetails {
                cast_time: 2,
                cooldown: 2,
                mana_cost: 10,
            },
        }
    }
}

#[derive(Resource)]
pub struct CooldownTimers(pub HashMap<Spell, Timer>);

fn update_cooldown_timers(time: Res<Time>, mut timers: ResMut<CooldownTimers>) {
    for (_spell, timer) in &mut timers.0 {
        timer.tick(time.delta());
    }
}

fn create_spray_fire_effect(effects: &mut ResMut<Assets<EffectAsset>>) -> Handle<EffectAsset> {
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

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(1.5).expr();
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

fn cast_spray_fire(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut q_player: Query<(Entity, &mut Player)>,
    player_state: Res<State<PlayerState>>,
    mut timers: ResMut<CooldownTimers>,
) {

    if !player_state.is_changed() {
        return;
    }

    let player_state = player_state.get();

    if *player_state != PlayerState::CastingSpell(Spell::SprayFire) {
        return;
    }

    let spray_fire_timer = timers.0.get_mut(&Spell::SprayFire);

    if let Some(timer) = spray_fire_timer {
        if timer.finished() {
            let cooldown_duration =
                Duration::from_secs(u64::from(Spell::SprayFire.details().cooldown));
            timer.set_duration(cooldown_duration);
            timer.reset();
        } else {
            return;
        }
    } else {
        let cooldown_duration =
            Duration::from_secs(u64::from(Spell::SprayFire.details().cooldown));

        timers.0.insert(
            Spell::SprayFire,
            Timer::new(cooldown_duration, TimerMode::Once),
        );
    };

    println!("Casting spray fire!");

    let effect = create_spray_fire_effect(&mut effects);
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
    let (player_id, _player) = q_player.get_single_mut().unwrap();
    commands.entity(player_id).push_children(&[fire_effect]);
}

pub struct PlayerSpellsPlugin;

impl Plugin for PlayerSpellsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CooldownTimers(HashMap::default()))
            .add_systems(Update, update_cooldown_timers)
            .add_systems(Update, cast_spray_fire);
    }
}
