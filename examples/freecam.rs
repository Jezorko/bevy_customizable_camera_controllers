#[path = "common/scene.rs"]
mod scene;

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_customizable_camera_controllers::fov::CameraFovController;
use bevy_customizable_camera_controllers::freecam::FreecamControllerPlugin;
use bevy_customizable_camera_controllers::movement::CameraMovementController;
use bevy_customizable_camera_controllers::rotation::CameraRotationController;
use scene::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_cursor_capture, setup_scene, spawn_camera))
        .add_plugins(FreecamControllerPlugin)
        .run();
}

fn setup_cursor_capture(mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    primary_cursor_options.visible = false;
    primary_cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        initial_camera_transform(),
        CameraMovementController::default(),
        CameraFovController::default(),
        CameraRotationController::default(),
    ));
}
