#[path = "common/scene.rs"]
mod scene;

use bevy::prelude::*;
use bevy_customizable_camera_controllers::rotation::*;
use scene::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_scene, spawn_camera))
        .add_systems(
            Update,
            rotate_camera_on::<ButtonInput<KeyCode>>(|input, time| {
                [
                    (KeyCode::KeyW, Vec2::NEG_Y),
                    (KeyCode::KeyS, Vec2::Y),
                    (KeyCode::KeyA, Vec2::NEG_X),
                    (KeyCode::KeyD, Vec2::X),
                ]
                .into_iter()
                .filter(|(key, _)| input.pressed(*key))
                .map(|(_, direction)| direction)
                .sum::<Vec2>()
                    * time.delta_secs()
            }),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        initial_camera_transform(),
        CameraRotationController::default(),
    ));
}
