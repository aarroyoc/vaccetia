use crate::distance::*;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
struct Wall;

const WALL_SCENE: &str = "wall.glb#Scene0";
const COLUMN_SCENE: &str = "column.glb#Scene0";

pub fn build_wall(
    mut commands: &mut Commands,
    server: &Res<AssetServer>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    origin: Vec3,
    dest: Vec3,
) {
    let wall = server.load(WALL_SCENE);
    let column = server.load(COLUMN_SCENE);
    let (x, z) = manhattan_distance(origin, dest);
    if x != 0.0 {
        for i in min_max_iter(origin.x, origin.x + x) {
            commands
                .spawn_bundle((
                    Transform::from_xyz(i, 0.0, origin.z)
                        .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
                    GlobalTransform::identity(),
                ))
                .with_children(|parent| {
                    parent.spawn_scene(wall.clone());
                })
                .insert(Wall);
        }
    }
    if z != 0.0 {
        for i in min_max_iter(origin.z, origin.z + z) {
            commands
                .spawn_bundle((
                    Transform::from_xyz(origin.x + x, 0.0, i),
                    GlobalTransform::identity(),
                ))
                .with_children(|parent| {
                    parent.spawn_scene(wall.clone());
                })
                .insert(Wall);
        }
    }
    // TODO: only checks for corners if they're built at the same time
    if x != 0.0 && z != 0.0 {
        commands
            .spawn_bundle((
                Transform::from_xyz(origin.x + x, 0.0, origin.z),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent.spawn_scene(column.clone());
            })
            .insert(Wall);
    }
}
