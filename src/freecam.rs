use crate::fov::{change_camera_fov_on, reset_fov};
use crate::movement::move_camera_on_keys;
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
                move_camera_on_keys(vec![
                    (KeyCode::KeyW, Transform::forward),
                    (KeyCode::KeyS, Transform::back),
                    (KeyCode::KeyA, Transform::left),
                    (KeyCode::KeyD, Transform::right),
                    (KeyCode::Space, Transform::up),
                    (KeyCode::ShiftLeft, Transform::down),
                ]),
                rotate_camera_on::<AccumulatedMouseMotion>(|input, time| {
                    input.delta * time.delta_secs()
                }),
                change_camera_fov_on::<AccumulatedMouseScroll>(|input, _| input.delta.y),
            ),
        );
    }
}
