use bevy::prelude::*;

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

pub fn move_camera_on_keys(
    controls: Vec<(KeyCode, fn(&Transform) -> Dir3)>,
) -> impl Fn(Res<ButtonInput<KeyCode>>, Res<Time>, Query<(&mut Transform, &CameraMovementController)>)
{
    move_camera_on::<ButtonInput<KeyCode>>(move |input, time, current_transform| {
        (&controls)
            .into_iter()
            .filter(|(key, _)| input.pressed(*key))
            .map(|(_, direction)| direction(current_transform))
            .map(|direction| direction.as_vec3())
            .sum::<Vec3>()
            * time.delta_secs()
    })
}

/// Changes camera position on [Input] by a delta calculated with [input_to_delta].
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
