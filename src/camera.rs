use bevy::prelude::*;

pub fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &PerspectiveProjection)>
) {
    for (mut transform_camera, _) in camera_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            let move_dir = transform_camera.left() * 0.1;
            transform_camera.translation += move_dir;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            let move_dir = transform_camera.right() * 0.1;
            transform_camera.translation += move_dir;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            let move_dir = transform_camera.forward() * 0.1;
            let old_y = transform_camera.translation.y;
            transform_camera.translation += move_dir;
            transform_camera.translation.y = old_y;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            let move_dir = transform_camera.back() * 0.1;
            let old_y = transform_camera.translation.y;
            transform_camera.translation += move_dir;
            transform_camera.translation.y = old_y;
        }

        if keyboard_input.pressed(KeyCode::Q) {
            let to_y = transform_camera.translation.y / -transform_camera.forward().y;
            let center_x = transform_camera.translation.x + transform_camera.forward().x * to_y;
            let center_z = transform_camera.translation.z + transform_camera.forward().z * to_y;
            let rotation = Quat::from_rotation_y(0.01);
            let point = Vec3::new(center_x, 0.0, center_z);
            transform_camera.translation = point + rotation * (transform_camera.translation - point);
            transform_camera.rotate(rotation);
            transform_camera.look_at(point, Vec3::Y);
        }
        if keyboard_input.pressed(KeyCode::E) {
            let to_y = transform_camera.translation.y / -transform_camera.forward().y;
            let center_x = transform_camera.translation.x + transform_camera.forward().x * to_y;
            let center_z = transform_camera.translation.z + transform_camera.forward().z * to_y;
            let rotation = Quat::from_rotation_y(-0.01);
            let point = Vec3::new(center_x, 0.0, center_z);
            transform_camera.translation = point + rotation * (transform_camera.translation - point);
            transform_camera.rotate(rotation);
            transform_camera.look_at(point, Vec3::Y);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let move_dir = transform_camera.forward() * 0.1;
            transform_camera.translation += move_dir;
        }
        if keyboard_input.pressed(KeyCode::X) {
            let move_dir = transform_camera.back() * 0.1;
            transform_camera.translation += move_dir;
        }

    }
}