//! Provides split screen support.
//!
//! Split screen is a feature that allows you to display two cameras side by side.
//! It is useful for games that have two players.
//!
//! ## Example
//! ```rust
//! use bevy::prelude::*;
//! use some_bevy_tools::split_screen;
//!
//! // Split screen requires a `LeftCamera` and a `RightCamera`.
//! fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     commands.spawn((Camera2dBundle::default(), split_screen::LeftCamera));
//!     commands.spawn((Camera2dBundle::default(), split_screen::RightCamera));
//! }
//!
//! App::new()
//!     //.add_plugins(DefaultPlugins)
//!     .add_plugins(split_screen::SplitScreenPlugin::default())
//!     .add_systems(Startup, setup);
//!     //.run();
//! ```

use bevy::{prelude::*, render::camera::Viewport, window::WindowResized};

/// Marker for the left camera.
///
/// It only works if exactly one LeftCamera is spawned in the scene.
#[derive(Component)]
pub struct LeftCamera;

/// Marker for the right camera.
///
/// It only works if exactly one RightCamera is spawned in the scene.
#[derive(Component)]
pub struct RightCamera;

/// Plugin for split screen support.
///
/// It only works if exactly one LeftCamera and one RightCamera are spawned in the scene.
#[derive(Default)]
pub struct SplitScreenPlugin;

impl Plugin for SplitScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set_camera_viewports_for_split_screen);
    }
}

// The following code is copied from the bevy split screen example at
// https://github.com/bevyengine/bevy/blob/latest/examples/3d/split_screen.rs
/// Set the camera viewports for split screen to lay out the cameras side by side.
fn set_camera_viewports_for_split_screen(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera: Query<&mut Camera, With<RightCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        if let (Ok(mut left_camera), Ok(mut right_camera)) =
            (left_camera.get_single_mut(), right_camera.get_single_mut())
        {
            left_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(
                    window.resolution.physical_width() / 2,
                    window.resolution.physical_height(),
                ),
                ..default()
            });
            left_camera.order = 1;

            right_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
                physical_size: UVec2::new(
                    window.resolution.physical_width() / 2,
                    window.resolution.physical_height(),
                ),
                ..default()
            });
            right_camera.order = 2;
        }
    }
}
