use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Component, Query, Res, Resource, Time, Transform};
use std::f32::consts::TAU;

#[derive(Component)]
pub struct CameraRotationController {
    pub turning_speed: Vec2,
    pub force_y_up_direction: bool,
}

impl Default for CameraRotationController {
    fn default() -> Self {
        Self {
            turning_speed: Vec2::splat(-TAU / 60.0),
            force_y_up_direction: true,
        }
    }
}

pub fn rotate_camera_on<Input: Resource>(
    input_to_delta: impl Fn(&Res<Input>, &Res<Time>) -> Vec2 + 'static,
) -> impl Fn(Res<Input>, Res<Time>, Query<(&mut Transform, &CameraRotationController)>) {
    move |input: Res<Input>,
          time: Res<Time>,
          mut query: Query<(&mut Transform, &CameraRotationController)>| {
        let input_delta = input_to_delta(&input, &time);
        for (mut transform, controller) in query.iter_mut() {
            let rotational_delta = controller.turning_speed * input_delta;

            transform.rotate_y(rotational_delta.x);
            transform.rotate_local_x(rotational_delta.y);

            if controller.force_y_up_direction {
                let y_forward_plane_normal = transform.forward().cross(Vec3::Y).normalize();

                let rejection = transform
                    .up()
                    .reject_from_normalized(y_forward_plane_normal);

                let rotation = Quat::from_rotation_arc(*transform.up(), rejection);
                transform.rotate(rotation);
            }
        }
    }
}
