use std::collections::VecDeque;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_systems(Startup, spawn_map_borders)
            .add_systems(Startup, spawn_tilemap)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell::<WallBundle>(1)
            //.add_systems(Update, cache_wall_locations);
            .add_systems(Update, spawn_wall_collision);
    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

//#[derive(Debug)]
//struct ColliderRectangle {
//    position: Vec3,
//    width: i32,
//    height: i32,
//}

//fn cache_wall_locations(
//    mut level_events: EventReader<LevelEvent>,
//    q_walls: Query<&GridCoords, With<Wall>>,
//    mut commands: Commands,
//) {
//    for level_event in level_events.read() {
//        if let LevelEvent::Spawned(level_iid) = level_event {
//            let walls = q_walls.into_iter().copied().collect::<Vec<_>>();
//            let lines = find_lines(walls);
//            println!("{:?}", lines);
//            for line in lines {
//                let collider_shape = if line.horizontal {
//                    (line.length as f32 * 8., 8.)
//                } else {
//                    (8., line.length as f32 * 8.)
//                };
//
//                commands.spawn((
//                    Collider::cuboid(collider_shape.0, collider_shape.1),
//                    TransformBundle::from_transform(Transform::from_translation(Vec3::new(
//                        line.position.0 as f32 * 17., line.position.1 as f32 * 17., 0.0,
//                    ))),
//                    RigidBody::KinematicPositionBased,
//                    ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
//                ));
//            }
//        }
//    }
//}

#[derive(Debug)]
struct Line {
    position: (i32, i32),
    length: i32,
    horizontal: bool,
}

//fn find_lines(wall_coords: Vec<GridCoords>) -> Vec<Line> {
//    let mut remaining: HashSet<GridCoords> = wall_coords.into_iter().collect();
//    let mut lines = Vec::new();
//
//    // add the coord as a line. if lines doesnt already have it
//
//    // if remaining has the same coords but with x + 1
//    // then add that coord
//
//    // Find horizontal lines
//    for &coord in &remaining.clone() {
//        if remaining.contains(&coord) {
//            let mut length = 1;
//            let mut x = coord.x + 1;
//            while remaining.contains(&GridCoords { x, y: coord.y }) {
//                remaining.remove(&GridCoords { x, y: coord.y });
//                length += 1;
//                x += 1;
//            }
//            if length > 1 {
//                lines.push(Line {
//                    position: (coord.x, coord.y),
//                    length,
//                    horizontal: true,
//                });
//                // Remove the starting coordinate of the line
//                remaining.remove(&coord);
//            }
//        }
//    }
//
//
//    lines
//}

#[allow(clippy::too_many_lines)]
fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.iter().for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.

        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert((
                                RigidBody::KinematicPositionBased,
                                ActiveCollisionTypes::default()
                                    | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemap/hills.ldtk"),
        ..Default::default()
    });
}

fn spawn_map_borders(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;

    commands.spawn((
        Collider::cuboid(1.0, 2600.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(10.0, 2600.0, 0.0))),
        RigidBody::KinematicPositionBased,
        ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
    ));
    commands.spawn((
        Collider::cuboid(10.0, 2600.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            5120.0, 2600.0, 0.0,
        ))),
    ));
    commands.spawn((
        Collider::cuboid(2600.0, 10.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(2610.0, 25.0, 0.0))),
    ));
    commands.spawn((
        Collider::cuboid(2600.0, 10.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            2610.0, 5120.0, 0.0,
        ))),
    ));
}
