use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use bevy_hanabi::prelude::*;

use super::{state::PlayerState, Player};

pub struct PlayerSpellsPlugin;
impl Plugin for PlayerSpellsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CooldownTimers::new())
            .insert_resource(CastingTimers::new())
            .add_systems(Update, (update_timers, despawn_melee_hitbox))
            .add_systems(
                Update,
                (
                    cast_spray_fire.run_if(in_state(PlayerState::CastingSpell(Spell::SprayFire))),
                    melee_attack.run_if(in_state(PlayerState::CastingSpell(Spell::Melee))),
                    cast_blazing_sword
                        .run_if(in_state(PlayerState::CastingSpell(Spell::BlazingSword))),
                )
                    .run_if(state_changed::<PlayerState>),
            );
    }
}

#[derive(Resource)]
pub struct CastingTimers(pub HashMap<Spell, Timer>);
impl CastingTimers {
    fn new() -> Self {
        let mut timers = HashMap::new();
        for spell in Spell::VALUES {
            let cast_time = spell.details().cast_time;
            let mut timer = Timer::from_seconds(cast_time, TimerMode::Once);
            // could this cause floating point errors? maybe use time + 1. ?
            timer.tick(Duration::from_secs_f32(cast_time));
            timers.insert(spell, timer);
        }
        Self(timers)
    }

    fn start_spell_casting_timer(&mut self, spell: Spell) {
        self.0.get_mut(&spell).unwrap().reset();
    }
}
#[derive(Resource)]
pub struct CooldownTimers(pub HashMap<Spell, Timer>);
impl CooldownTimers {
    fn new() -> Self {
        let mut timers = HashMap::new();
        for spell in Spell::VALUES {
            let cooldown_time = spell.details().cooldown;
            let mut timer = Timer::from_seconds(cooldown_time, TimerMode::Once);
            // could this cause floating point errors? maybe use time + 1. ?
            timer.tick(Duration::from_secs_f32(cooldown_time));
            timers.insert(spell, timer);
        }
        Self(timers)
    }

    fn start_spell_cooldown_timer(&mut self, spell: Spell) {
        self.0.get_mut(&spell).unwrap().reset();
    }
}

fn update_timers(
    time: Res<Time>,
    mut cooldown_timers: ResMut<CooldownTimers>,
    mut casting_timers: ResMut<CastingTimers>,
) {
    for (_spell, timer) in &mut cooldown_timers.0 {
        timer.tick(time.delta());
    }
    for (_spell, timer) in &mut casting_timers.0 {
        timer.tick(time.delta());
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Component)]
pub struct SpellDetails<'a> {
    pub cast_time: f32,
    pub cooldown: f32,
    pub mana_cost: u32,
    pub ui_icon: &'a str,
}

#[derive(Debug, PartialEq, Clone, Copy, Component, Hash, Eq)]
pub enum Spell {
    SprayFire,
    BlazingSword,
    Melee,
}
impl Spell {
    pub const VALUES: [Self; 3] = [Self::SprayFire, Self::BlazingSword, Self::Melee];
}

impl Spell {
    pub fn details(self) -> SpellDetails<'static> {
        match self {
            Spell::SprayFire => SpellDetails {
                cast_time: 1.,
                cooldown: 1.,
                mana_cost: 10,
                ui_icon: "skill_icons/FireMage_17.png",
            },
            Spell::BlazingSword => SpellDetails {
                cast_time: 2.,
                cooldown: 2.,
                mana_cost: 10,
                ui_icon: "skill_icons/FireMage_20.png",
            },
            Spell::Melee => SpellDetails {
                cast_time: 1.,
                cooldown: 3.,
                mana_cost: 0,
                ui_icon: "skill_icons/FireMage_29.png",
            },
        }
    }

    pub fn is_casting_finished(self, casting_timers: &CastingTimers) -> bool {
        casting_timers.0.get(&self).unwrap().finished()
    }

    pub fn is_cooldown_finished(self, cooldown_timers: &CooldownTimers) -> bool {
        cooldown_timers.0.get(&self).unwrap().finished()
    }
}

fn melee_attack(
    mut commands: Commands,
    mut cooldown_timers: ResMut<CooldownTimers>,
    mut casting_timers: ResMut<CastingTimers>,
    q_player: Query<(Entity, &Sprite), With<Player>>,
) {
    let (player_entity, player_sprite) = q_player.single();
    create_melee_hitbox(
        &mut commands,
        player_entity,
        player_sprite,
        Spell::Melee.details().cast_time,
    );

    cooldown_timers.start_spell_cooldown_timer(Spell::Melee);
    casting_timers.start_spell_casting_timer(Spell::Melee);
}

#[derive(Component)]
pub struct PlayerMeleeHitbox(Timer);

fn create_melee_hitbox(
    commands: &mut Commands,
    player_entity: Entity,
    player_sprite: &Sprite,
    despawn_after_secs: f32,
) {
    let mut transform = Transform::from_translation(Vec3::new(36., -24., 0.));
    if player_sprite.flip_x {
        transform.translation.x *= -1.;
    }
    let melee_hitbox = commands
        .spawn((
            PlayerMeleeHitbox(Timer::from_seconds(despawn_after_secs, TimerMode::Once)),
            Collider::cuboid(30., 10.),
            TransformBundle::from_transform(transform),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        ))
        .id();

    commands
        .entity(player_entity)
        .push_children(&[melee_hitbox]);
}

fn despawn_melee_hitbox(
    mut commands: Commands,
    mut q_hitbox: Query<(Entity, &mut PlayerMeleeHitbox)>,
    time: Res<Time>,
) {
    if let Ok((hitbox_entity, mut hitbox_component)) = q_hitbox.get_single_mut() {
        hitbox_component.0.tick(time.delta());
        if hitbox_component.0.finished() {
            commands.entity(hitbox_entity).despawn();
        }
    };
}

fn cast_spray_fire(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut q_player: Query<(Entity, &mut Player)>,
    mut cooldown_timers: ResMut<CooldownTimers>,
) {
    let effect = create_fire_cone_effect(&mut effects);
    let fire_effect = commands
        .spawn((
            Name::new("emit:rate"),
            ParticleEffectBundle {
                effect: ParticleEffect::new(effect),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                    .with_rotation(Quat::from_rotation_z(1.2)),
                ..Default::default()
            },
        ))
        .id();
    let (player_id, _player) = q_player.get_single_mut().unwrap();
    commands.entity(player_id).push_children(&[fire_effect]);

    cooldown_timers.start_spell_cooldown_timer(Spell::SprayFire);
}

fn cast_blazing_sword(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut q_player: Query<(Entity, &mut Player)>,
    mut cooldown_timers: ResMut<CooldownTimers>,
) {
    let effect = create_fire_cone_effect(&mut effects);
    let fire_effect = commands
        .spawn((
            Name::new("emit:rate"),
            ParticleEffectBundle {
                effect: ParticleEffect::new(effect),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                    .with_rotation(Quat::from_rotation_z(-1.2)),
                ..Default::default()
            },
        ))
        .id();
    let (player_id, _player) = q_player.get_single_mut().unwrap();
    commands.entity(player_id).push_children(&[fire_effect]);

    cooldown_timers.start_spell_cooldown_timer(Spell::BlazingSword);
}

fn create_fire_cone_effect(effects: &mut ResMut<Assets<EffectAsset>>) -> Handle<EffectAsset> {
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
