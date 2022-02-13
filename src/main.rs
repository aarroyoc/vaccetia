use std::cmp::min;
use bevy::input::ElementState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_mod_raycast::*;

const GRID_SIZE: u32 = 1;

#[derive(Component)]
struct Pointer;

#[derive(Component)]
struct StartMarker;

#[derive(Component)]
struct MarkerWall;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Vaccetia".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4})
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<RaycastSet>::default())
        .add_system_to_stage(
            CoreStage::PreUpdate,
            update_raycast.before(RaycastSystem::BuildRays)
        )
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(update_pointer.after(RaycastSystem::BuildRays))
                .with_system(click_wall.after(RaycastSystem::BuildRays))
        )
        .add_system(build_pre_walls)
        .run();
}

struct RaycastSet;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0})),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    })
        .insert(RayCastMesh::<RaycastSet>::default());

    // grid
    /*let texture_grid = asset_server.load("grid.png");
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0})),
        material: materials.add(texture_grid.clone().into()),
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0})),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });*/

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0,2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
        .insert(RayCastSource::<RaycastSet>::default());

    // UI
    /*commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        });*/


    // pointer
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 32
        })),
        material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
        ..Default::default()
    })
        .insert(Pointer);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 32
        })),
        material: materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
        visibility: Visibility {
            is_visible: false
        },
        ..Default::default()
    }).insert(StartMarker);
}

fn update_raycast(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<RaycastSet>>
) {
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return
    };

    for mut pick_source in query.iter_mut() {{
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }}
}

fn update_pointer(
    mut query: Query<&mut RayCastSource<RaycastSet>>,
    mut pointer_query: Query<&mut GlobalTransform, With<Pointer>>,
) {
    for raycast_source in query.iter() {
        match raycast_source.intersect_top() {
            Some(intersection) => {
                for mut transform_pointer in pointer_query.iter_mut() {
                    let position = intersection.1.position();
                    let position_fixed = Vec3::new(
                        position.x.round(),
                        position.y.round(),
                        position.z.round()
                    );
                    transform_pointer.translation = position_fixed;
                }
            }
            None => return
        }
    }
}

fn build_pre_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pointer_query: Query<&GlobalTransform, (With<Pointer>, Without<StartMarker>)>,
    marker_query: Query<(&GlobalTransform, &Visibility), With<StartMarker>>,
    marker_wall_query: Query<Entity, With<MarkerWall>>
) {
    for marker_wall in marker_wall_query.iter() {
        commands.entity(marker_wall).despawn();
    }
    for (pointer) in pointer_query.iter() {
        for (transform_marker, visibility_marker) in marker_query.iter() {
            if visibility_marker.is_visible {
                let (x, z) = manhattan_distance(transform_marker.translation, pointer.translation);
                if x != 0.0 {
                    let (min_x, max_x) = min_max(transform_marker.translation.x, transform_marker.translation.x + x);
                    commands.spawn_bundle(PbrBundle {
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
                    }).insert(MarkerWall);
                }
                if z != 0.0 {
                    let (min_x, max_x) = min_max(transform_marker.translation.x + x - 0.1, transform_marker.translation.x + x + 0.1);
                    let (min_z, max_z) = min_max(transform_marker.translation.z, transform_marker.translation.z + z);
                    commands.spawn_bundle(PbrBundle {
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
                    }).insert(MarkerWall);
                }
            }
        }
    }
}

fn min_max(
    a: f32,
    b: f32
) -> (f32, f32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

fn manhattan_distance(
    origin: Vec3,
    dest: Vec3
) -> (f32, f32) {
    (dest.x - origin.x, dest.z - origin.z)
}

fn click_wall(
    mut events: EventReader<MouseButtonInput>,
    mut pointer_query: Query<(&mut GlobalTransform), (With<Pointer>, Without<StartMarker>)>,
    mut marker_query: Query<(&mut GlobalTransform, &mut Visibility), With<StartMarker>>
) {
    if let Some(event) = events.iter().last() {
        if event.button == MouseButton::Left && event.state == ElementState::Released {
            for (transform_pointer) in pointer_query.iter_mut() {
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