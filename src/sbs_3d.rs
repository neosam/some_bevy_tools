//! Support for 3D output in SBS format for 3D glasses.

use crate::split_screen;
use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct SbsCameraGap {
    pub gap: f32,
}
#[derive(Component, Debug, Default)]
pub enum SbsCameraState {
    #[default]
    SBS,
    Deactivated,
}

#[derive(Bundle)]
pub struct SbsCameraBundle {
    pub sbs_camera_gap: SbsCameraGap,
    pub sbs_camera_state: SbsCameraState,
    pub transform: Transform,
}

impl SbsCameraBundle {
    pub fn from_transform_and_gap(transform: Transform, gap: f32) -> Self {
        Self {
            transform,
            sbs_camera_gap: SbsCameraGap { gap },
            sbs_camera_state: SbsCameraState::SBS,
        }
    }
}

/// A system which recalculates the position of the left and right camera.
#[allow(clippy::type_complexity)]
pub fn update_sbs_camera_transform(
    sbs_camera: Query<
        (&SbsCameraGap, &Transform),
        (
            Or<(Changed<SbsCameraGap>, Changed<Transform>)>,
            Without<split_screen::LeftCamera>,
            Without<split_screen::RightCamera>,
        ),
    >,
    mut left_camera: Query<
        &mut Transform,
        (
            With<split_screen::LeftCamera>,
            Without<split_screen::RightCamera>,
        ),
    >,
    mut right_camera: Query<&mut Transform, With<split_screen::RightCamera>>,
) {
    if let (Ok((sbs_camera, sbs_camera_transform)), Ok(mut left_camera), Ok(mut right_camera)) = (
        sbs_camera.get_single(),
        left_camera.get_single_mut(),
        right_camera.get_single_mut(),
    ) {
        let gap = sbs_camera.gap;
        let left_translation = sbs_camera_transform.left() * gap / 2.0;
        *left_camera = *sbs_camera_transform;
        left_camera.translation += left_translation;

        let right_translation = sbs_camera_transform.right() * gap / 2.0;
        *right_camera = *sbs_camera_transform;
        right_camera.translation += right_translation;
    }
}

pub fn sbs_camera_state_update() {}

pub struct Sbs3DPlugin;

impl Plugin for Sbs3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(split_screen::SplitScreenPlugin)
            .add_systems(Update, update_sbs_camera_transform);
    }
}
