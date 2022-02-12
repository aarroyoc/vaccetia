use bevy::prelude::*;
use bevy_mod_raycast::*;

const GRID_SIZE: u32 = 1;

#[derive(Component)]
struct Pointer;

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
        .add_system_to_stage(
            CoreStage::PreUpdate,
            update_pointer.after(RaycastSystem::BuildRays)
        )
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
    let texture_grid = asset_server.load("grid.png");
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
    });

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
    mut pointer: Query<&mut GlobalTransform, With<Pointer>>
) {
    for raycast_source in query.iter() {
        match raycast_source.intersect_top() {
            Some(intersection) => {
                for mut transform in pointer.iter_mut() {
                    let position = intersection.1.position();
                    let position_fixed = Vec3::new(
                        position.x.round(),
                        position.y.round(),
                        position.z.round()
                    );
                    transform.translation = position_fixed;
                }
            }
            None => return
        }
    }
}