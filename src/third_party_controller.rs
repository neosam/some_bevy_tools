use std::f32::consts::PI;

use bevy::{
    input::mouse::{self, MouseWheel},
    prelude::*,
};

use crate::third_party_camera;

#[derive(Component)]
pub struct ThirdPartyController {
    pub min_distance: f32,
    pub max_distance: f32,
}

pub fn third_party_camera_controller_system(
    mut scroll_events: EventReader<MouseWheel>,
    mut motion_events: EventReader<mouse::MouseMotion>,
    mut third_party_query: Query<(
        &ThirdPartyController,
        &mut third_party_camera::ThirdPartyCamera,
    )>,
) {
    bevy::log::info!("third_party-camera-controller-system");
    for ev in scroll_events.read() {
        let offset = match ev.unit {
            mouse::MouseScrollUnit::Line => ev.y * 10.0,
            mouse::MouseScrollUnit::Pixel => ev.y,
        };
        for (controller, mut camera) in third_party_query.iter_mut() {
            camera.distance =
                (camera.distance - offset).clamp(controller.min_distance, controller.max_distance);
        }
    }

    for ev in motion_events.read() {
        for (_controller, mut camera) in third_party_query.iter_mut() {
            camera.rotate_y -= ev.delta.x * 0.005;
            camera.rotate_x =
                (camera.rotate_x - ev.delta.y * -0.005).clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);
        }
    }
}

pub struct ThirdPartyControllerPlugin;
impl Plugin for ThirdPartyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(third_party_camera::ThirdPartyCameraPlugin)
            .add_systems(Update, third_party_camera_controller_system);
    }
}
