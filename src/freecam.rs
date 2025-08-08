use crate::fov::{change_camera_fov_on, reset_fov};
use crate::movement::move_camera_on;
use crate::rotation::rotate_camera_on;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

/// A simple controller for a camera that is not attached to any entity.
/// Keybindings:
///  * W, S, A, D, SPACE, LEFT SHIFT - move camera
///  * mouse - rotate camera
///  * mouse wheel - change camera FOV
#[derive(Default)]
pub struct FreecamControllerPlugin;

impl Plugin for FreecamControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, reset_fov).add_systems(
            Update,
            (
                move_camera_on::<ButtonInput<KeyCode>>(|input, time, current_transform| {
                    [
                        (KeyCode::KeyW, current_transform.forward()),
                        (KeyCode::KeyS, current_transform.back()),
                        (KeyCode::KeyA, current_transform.left()),
                        (KeyCode::KeyD, current_transform.right()),
                        (KeyCode::Space, current_transform.up()),
                        (KeyCode::ShiftLeft, current_transform.down()),
                    ]
                    .into_iter()
                    .filter(|(key, _)| input.pressed(*key))
                    .map(|(_, direction)| direction)
                    .map(|direction| direction.as_vec3())
                    .sum::<Vec3>()
                        * time.delta_secs()
                }),
                rotate_camera_on::<AccumulatedMouseMotion>(|input, time| {
                    input.delta * time.delta_secs()
                }),
                change_camera_fov_on::<AccumulatedMouseScroll>(|input, _| input.delta.y),
            ),
        );
    }
}
