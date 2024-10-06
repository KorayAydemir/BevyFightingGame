use bevy::prelude::*;

use crate::common::sprite::{update_spritesheet_indices, AnimationTimer};

use self::state::KoalaState;

mod sprite;
mod state;

const KOALA_SCALE: f32 = 1.5;

pub struct NeutralKoalaPlugin;
impl Plugin for NeutralKoalaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(state::KoalaStatePlugin)
            .add_plugins(sprite::KoalaSpritePlugin)
            .add_systems(Startup, spawn_koala)
            .add_systems(Update, animate_koala);
    }
}

fn koala_sprite_indices(state: &KoalaState) -> (usize, usize) {
    match state {
        KoalaState::Idling => (0, 11),
        KoalaState::Moving(_) => (12, 23),
    }
}

#[derive(Component)]
struct Koala;

fn spawn_koala(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut res_texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 12, 4, None, None);
    let texture_atlas_handle = res_texture_atlas_layouts.add(texture_atlas);

    commands.spawn((
        Koala,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(50., 50., 0.))
                .with_scale(Vec3::splat(KOALA_SCALE)),
            atlas: TextureAtlas {
                layout: texture_atlas_handle,
                index: 0,
            },
            texture: asset_server.load("sprites/koala.png"),
            ..default()
        },
    ));
}

fn animate_koala(
    time: Res<Time>,
    mut q_koala: Query<(&mut AnimationTimer, &mut TextureAtlas), With<Koala>>,
    res_koala_state: Res<State<KoalaState>>,
) {
    let (animation_timer, atlas) = q_koala.get_single_mut().unwrap();
    let koala_state = res_koala_state.get();
    let indices = koala_sprite_indices(koala_state);

    update_spritesheet_indices(&time, animation_timer, atlas, res_koala_state, indices);
}
