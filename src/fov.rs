use bevy::log::debug;
use bevy::prelude::{Component, Projection, Query, Res, Resource, Time};
use std::ops::RangeInclusive;

#[derive(Component)]
pub struct CameraFovController {
    pub input_scale: f32,
    pub fov_degrees_range: RangeInclusive<f32>,
    pub negate_delta: bool,
}

impl Default for CameraFovController {
    fn default() -> Self {
        Self {
            input_scale: 1.0,
            fov_degrees_range: 45.0..=135.0,
            negate_delta: true,
        }
    }
}

pub fn reset_fov(query: Query<(&mut Projection, &CameraFovController)>) {
    update_fov(query, |current_fov_degrees, _| current_fov_degrees);
}

pub fn change_camera_fov_on<Input: Resource>(
    input_to_delta: impl Fn(&Res<Input>, &Res<Time>) -> f32 + 'static,
) -> impl Fn(Res<Input>, Res<Time>, Query<(&mut Projection, &CameraFovController)>) {
    move |input, time, query| {
        let input_delta = input_to_delta(&input, &time);
        if input_delta != 0.0 {
            update_fov(query, |current_fov_degrees, camera| {
                current_fov_degrees
                    + (camera.input_scale
                        * input_delta
                        * if camera.negate_delta { -1.0 } else { 1.0 })
            });
        }
    }
}

fn update_fov(
    mut query: Query<(&mut Projection, &CameraFovController)>,
    fov_degrees_transformer: impl Fn(f32, &CameraFovController) -> f32,
) {
    for (mut projection, camera) in query.iter_mut() {
        if let Projection::Perspective(perspective) = &mut *projection {
            let current_fov_degrees = perspective.fov.to_degrees();
            let new_fov_degrees = fov_degrees_transformer(current_fov_degrees, camera).clamp(
                *camera.fov_degrees_range.start(),
                *camera.fov_degrees_range.end(),
            );
            debug!("changing fov to: {new_fov_degrees} degrees");
            perspective.fov = new_fov_degrees.to_radians();
        }
    }
}
