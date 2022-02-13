use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use bevy::prelude::*;
use bevy_mod_raycast::*;

#[derive(Component)]
pub struct Pointer;

#[derive(Component)]
pub struct StartMarker;

#[derive(Component)]
pub struct MarkerWall;

pub struct RaycastSet;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // pointer
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 32,
            })),
            material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
            ..Default::default()
        })
        .insert(Pointer);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 32,
            })),
            material: materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(StartMarker);
}

pub fn update_raycast(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<RaycastSet>>,
) {
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in query.iter_mut() {
        {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
        }
    }
}

pub fn update_pointer(
    query: Query<&RayCastSource<RaycastSet>>,
    mut pointer_query: Query<&mut GlobalTransform, With<Pointer>>,
) {
    for raycast_source in query.iter() {
        match raycast_source.intersect_top() {
            Some(intersection) => {
                for mut transform_pointer in pointer_query.iter_mut() {
                    let position = intersection.1.position();
                    let position_fixed =
                        Vec3::new(position.x.round(), position.y.round(), position.z.round());
                    transform_pointer.translation = position_fixed;
                }
            }
            None => return,
        }
    }
}

pub fn click_wall(
    mut events: EventReader<MouseButtonInput>,
    mut pointer_query: Query<&mut GlobalTransform, (With<Pointer>, Without<StartMarker>)>,
    mut marker_query: Query<(&mut GlobalTransform, &mut Visibility), With<StartMarker>>,
) {
    if let Some(event) = events.iter().last() {
        if event.button == MouseButton::Left && event.state == ElementState::Released {
            for transform_pointer in pointer_query.iter_mut() {
                for (mut transform_marker, mut visibility_marker) in marker_query.iter_mut() {
                    if visibility_marker.is_visible {
                        visibility_marker.is_visible = false;
                    } else {
                        transform_marker.translation.x = transform_pointer.translation.x;
                        transform_marker.translation.y = transform_pointer.translation.y;
                        transform_marker.translation.z = transform_pointer.translation.z;
                        visibility_marker.is_visible = true;
                    }
                }
            }
        }
    }
}

pub fn build_pre_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pointer_query: Query<&GlobalTransform, (With<Pointer>, Without<StartMarker>)>,
    marker_query: Query<(&GlobalTransform, &Visibility), With<StartMarker>>,
    marker_wall_query: Query<Entity, With<MarkerWall>>,
) {
    for marker_wall in marker_wall_query.iter() {
        commands.entity(marker_wall).despawn();
    }
    for pointer in pointer_query.iter() {
        for (transform_marker, visibility_marker) in marker_query.iter() {
            if visibility_marker.is_visible {
                let (x, z) = manhattan_distance(transform_marker.translation, pointer.translation);
                if x != 0.0 {
                    let (min_x, max_x) = min_max(
                        transform_marker.translation.x,
                        transform_marker.translation.x + x,
                    );
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box {
                                min_x,
                                max_x,
                                min_y: -0.1,
                                max_y: 0.1,
                                min_z: transform_marker.translation.z - 0.1,
                                max_z: transform_marker.translation.z + 0.1,
                            })),
                            material: materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
                            ..Default::default()
                        })
                        .insert(MarkerWall);
                }
                if z != 0.0 {
                    let (min_x, max_x) = min_max(
                        transform_marker.translation.x + x - 0.1,
                        transform_marker.translation.x + x + 0.1,
                    );
                    let (min_z, max_z) = min_max(
                        transform_marker.translation.z,
                        transform_marker.translation.z + z,
                    );
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box {
                                min_x,
                                max_x,
                                min_y: -0.1,
                                max_y: 0.1,
                                min_z,
                                max_z,
                            })),
                            material: materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
                            ..Default::default()
                        })
                        .insert(MarkerWall);
                }
            }
        }
    }
}

fn min_max(a: f32, b: f32) -> (f32, f32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

fn manhattan_distance(origin: Vec3, dest: Vec3) -> (f32, f32) {
    (dest.x - origin.x, dest.z - origin.z)
}
