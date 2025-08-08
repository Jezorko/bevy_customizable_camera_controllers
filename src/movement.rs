use bevy::prelude::{Component, Query, Res, Resource, Time, Transform, Vec3};

#[derive(Component)]
pub struct CameraMovementController {
    pub movement_speed: Vec3,
}

impl Default for CameraMovementController {
    fn default() -> Self {
        Self {
            movement_speed: Vec3::ONE,
        }
    }
}

pub fn move_camera_on<Input: Resource>(
    input_to_delta: impl Fn(&Res<Input>, &Res<Time>, &Transform) -> Vec3 + 'static,
) -> impl Fn(Res<Input>, Res<Time>, Query<(&mut Transform, &CameraMovementController)>) {
    move |input, time, mut query: Query<(&mut Transform, &CameraMovementController)>| {
        for (mut transform, controller) in query.iter_mut() {
            let input_delta = input_to_delta(&input, &time, &transform);
            transform.translation += input_delta * controller.movement_speed;
        }
    }
}
