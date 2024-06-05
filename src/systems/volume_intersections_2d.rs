use bevy::prelude::*;
use bevy_math::bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume, IntersectsVolume};

#[derive(Default, States, Debug, Hash, Clone, Eq, PartialEq)]
enum Test {
    AabbSweep,
    CircleSweep,
    #[default]
    RayCast,
    AabbCast,
    CircleCast,
}

#[derive(Component)]
enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle2d),
    Line(Segment2d),
    Capsule(Capsule2d),
    Polygon(RegularPolygon),
}

#[derive(Component)]
enum DesiredVolume {
    Aabb,
    Circle,
}

#[derive(Component, Debug)]
enum CurrentVolume {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}

#[derive(Component, Deref, DerefMut, Default)]
struct Intersects(bool);

#[derive(Component)]
struct Spin;

const OFFSET_X: f32 = 125.;
const OFFSET_Y: f32 = 75.;

pub fn setup(mut commands: Commands, loader: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
            ..default()
        },
        Shape::Circle(Circle::new(45.)),
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0., OFFSET_Y, 0.),
            ..default()
        },
        Shape::Rectangle(Rectangle::new(80., 80.)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(OFFSET_X, OFFSET_Y, 0.),
            ..default()
        },
        Shape::Triangle(Triangle2d::new(
            Vec2::new(-40., -40.),
            Vec2::new(-20., 40.),
            Vec2::new(40., 50.),
        )),
        Spin,
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-OFFSET_X, -OFFSET_Y, 0.),
            ..default()
        },
        Shape::Line(Segment2d::new(Direction2d::from_xy(1., 0.3).unwrap(), 90.)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0., -OFFSET_Y, 0.),
            ..default()
        },
        Shape::Capsule(Capsule2d::new(25., 50.)),
        Spin,
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(OFFSET_X, -OFFSET_Y, 0.),
            ..default()
        },
        Shape::Polygon(RegularPolygon::new(50., 6)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 26.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn update_text(mut text: Query<&mut Text>, cur_state: Res<State<Test>>) {
    if !cur_state.is_changed() {
        return;
    }

    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    text.clear();

    text.push_str("Intersection test:\n");
    use Test::*;
    for test in &[AabbSweep, CircleSweep, RayCast, AabbCast, CircleCast] {
        let s = if **cur_state == *test { "*" } else { " " }; // TODO try removing one pointer from cur_state and see if it causes bug
        text.push_str(&format!(" {s} {test:?} {s}\n"));
    }
    text.push_str("\npress Space to cycle");
}

fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() / 5.);
    }
}

fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
    let color = Color::GRAY;

    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;

        match shape {
            Shape::Rectangle(r) => {
                gizmos.primitive_2d(*r, translation, rotation, color);
            }
            Shape::Circle(c) => {
                gizmos.primitive_2d(*c, translation, rotation, color);
            }
            Shape::Triangle(t) => {
                gizmos.primitive_2d(*t, translation, rotation, color);
            }
            Shape::Line(l) => {
                gizmos.primitive_2d(*l, translation, rotation, color);
            }
            Shape::Capsule(c) => {
                gizmos.primitive_2d(*c, translation, rotation, color);
            }
            Shape::Polygon(p) => {
                gizmos.primitive_2d(*p, translation, rotation, color);
            }
        }
    }
}

fn render_volumes(mut gizmos: Gizmos, query: Query<(&CurrentVolume, &Intersects)>) {
    for (volume, intersects) in query.iter() {
        let color = if **intersects {
            Color::CYAN
        } else {
            Color::ORANGE_RED
        };
        match volume {
            CurrentVolume::Aabb(a) => {
                gizmos.rect_2d(a.center(), 0., a.half_size() * 2., color);
            }
            CurrentVolume::Circle(c) => {
                gizmos.circle_2d(c.center(), c.radius(), color);
            }
        }
    }
}

fn update_volumes(
    mut commands: Commands,
    query: Query<
        (Entity, &DesiredVolume, &Shape, &Transform),
        Or<(Changed<DesiredVolume>, Changed<Shape>, Changed<Transform>)>,
    >,
) {
    for (entity, desired_volume, shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        match desired_volume {
            DesiredVolume::Aabb => {
                let aabb = match shape {
                    Shape::Rectangle(r) => r.aabb_2d(translation, rotation),
                    Shape::Circle(c) => c.aabb_2d(translation, rotation),
                    Shape::Triangle(t) => t.aabb_2d(translation, rotation),
                    Shape::Line(l) => l.aabb_2d(translation, rotation),
                    Shape::Capsule(c) => c.aabb_2d(translation, rotation),
                    Shape::Polygon(p) => p.aabb_2d(translation, rotation),
                };
                commands.entity(entity).insert(CurrentVolume::Aabb(aabb));
            }
            DesiredVolume::Circle => {
                let circle = match shape {
                    Shape::Rectangle(r) => r.bounding_circle(translation, rotation),
                    Shape::Circle(c) => c.bounding_circle(translation, rotation),
                    Shape::Triangle(t) => t.bounding_circle(translation, rotation),
                    Shape::Line(l) => l.bounding_circle(translation, rotation),
                    Shape::Capsule(c) => c.bounding_circle(translation, rotation),
                    Shape::Polygon(p) => p.bounding_circle(translation, rotation),
                };
                commands
                    .entity(entity)
                    .insert(CurrentVolume::Circle(circle));
            }
        }
    }
}

fn get_intersection_position(time: &Time) -> Vec2 {
    let x = (0.8 * time.elapsed_seconds()).cos() * 250.;
    let y = (0.4 * time.elapsed_seconds()).cos() * 100.;
    Vec2::new(x, y)
}

fn aabb_intersection_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    let center = get_intersection_position(&time);
    let aabb = Aabb2d::new(center, Vec2::splat(50.));

    gizmos.rect_2d(center, 0., aabb.half_size() * 2., Color::YELLOW);

    for (volume, mut intersects) in volumes.iter_mut() {
        let hit = match volume {
            CurrentVolume::Aabb(a) => aabb.intersects(a),
            CurrentVolume::Circle(c) => aabb.intersects(c),
        };
        **intersects = hit;
    }
}

fn update_test_state(
    keycode: Res<ButtonInput<KeyCode>>,
    cur_state: Res<State<Test>>,
    mut state: ResMut<NextState<Test>>,
) {
    if !keycode.just_pressed(KeyCode::Space) {
        return;
    }

    use Test::*;
    let next = match **cur_state {
        AabbSweep => CircleSweep,
        CircleSweep => RayCast,
        RayCast => AabbCast,
        AabbCast => CircleCast,
        CircleCast => AabbSweep,
    };
    state.set(next);
}

pub struct VolumeIntersections2dPlugin;

impl Plugin for VolumeIntersections2dPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Test>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (update_text, spin, update_volumes, update_test_state),
            )
            .add_systems(
                PostUpdate,
                (
                    render_shapes,
                    (aabb_intersection_system.run_if(in_state(Test::AabbSweep)),),
                    render_volumes,
                ),
            );
    }
}
