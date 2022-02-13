use bevy::prelude::*;
use bevy_mod_raycast::*;

mod editor;

const GRID_SIZE: u32 = 1;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Vaccetia".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup)
        .add_startup_system(editor::setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<editor::RaycastSet>::default())
        .add_system_to_stage(
            CoreStage::PreUpdate,
            editor::update_raycast.before(RaycastSystem::BuildRays),
        )
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(editor::update_pointer.after(RaycastSystem::BuildRays))
                .with_system(editor::click_wall.after(RaycastSystem::BuildRays)),
        )
        .add_system(editor::build_pre_walls)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(RayCastMesh::<editor::RaycastSet>::default());

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

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(RayCastSource::<editor::RaycastSet>::default());

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
}
