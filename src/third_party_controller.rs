use std::f32::consts::PI;

use bevy::prelude::*;

use crate::input::SliderMappingType;
use crate::{input, third_party_camera};

#[derive(Component)]
pub struct ThirdPartyController {
    pub min_distance: f32,
    pub max_distance: f32,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum CharacterControllerEvent {
    Turn,
    IncreaseCameraDistance,
    DecreaseCameraDistance,
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

pub fn default_character_controller_event_mapping() -> input::InputMapping<CharacterControllerEvent>
{
    input::InputMapping::from((
        [
            (
                input::UserButtonInput::MouseScrollDown,
                CharacterControllerEvent::IncreaseCameraDistance,
            ),
            (
                input::UserButtonInput::MouseScrollUp,
                CharacterControllerEvent::DecreaseCameraDistance,
            ),
            (
                input::UserButtonInput::KeyPressed(KeyCode::KeyW),
                CharacterControllerEvent::MoveForward,
            ),
            (
                input::UserButtonInput::KeyPressed(KeyCode::KeyS),
                CharacterControllerEvent::MoveBackward,
            ),
            (
                input::UserButtonInput::KeyPressed(KeyCode::KeyA),
                CharacterControllerEvent::MoveLeft,
            ),
            (
                input::UserButtonInput::KeyPressed(KeyCode::KeyD),
                CharacterControllerEvent::MoveRight,
            ),
        ],
        [(
            SliderMappingType::MouseMove(10.0),
            CharacterControllerEvent::Turn,
            0.005,
            -0.005,
        )],
    ))
}

pub fn third_party_camera_controller_system(
    mut action_events: EventReader<input::ActionEvent<CharacterControllerEvent>>,
    mut slider_events: EventReader<input::DirectionSliderEvent<CharacterControllerEvent>>,
    mut third_party_query: Query<(
        &ThirdPartyController,
        &mut third_party_camera::ThirdPartyCamera,
    )>,
) {
    for ev in action_events.read() {
        match ev.action {
            CharacterControllerEvent::IncreaseCameraDistance => {
                let offset = 1.0;
                for (controller, mut camera) in third_party_query.iter_mut() {
                    camera.distance = (camera.distance - offset)
                        .clamp(controller.min_distance, controller.max_distance);
                }
            }
            CharacterControllerEvent::DecreaseCameraDistance => {
                let offset = -1.0;
                for (controller, mut camera) in third_party_query.iter_mut() {
                    camera.distance = (camera.distance - offset)
                        .clamp(controller.min_distance, controller.max_distance);
                }
            }
            _ => (),
        }
    }

    for ev in slider_events.read() {
        for (_controller, mut camera) in third_party_query.iter_mut() {
            camera.rotate_y -= ev.x;
            camera.rotate_x = (camera.rotate_x - ev.y).clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);
        }
    }
}

pub fn move_controller_plane(
    camera_query: Query<&third_party_camera::ThirdPartyCamera, With<ThirdPartyController>>,
    mut target_query: Query<&mut Transform, Without<ThirdPartyController>>,
    mut action_events: EventReader<input::ActionEvent<CharacterControllerEvent>>,
    time: Res<Time>,
) {
    for ev in action_events.read() {
        match ev.action {
            CharacterControllerEvent::MoveForward => {
                for camera in camera_query.iter() {
                    if let Ok(mut target_transform) = target_query.get_mut(camera.target) {
                        let rotation_y = camera.rotate_y;
                        target_transform.translation.x -=
                            rotation_y.sin() * 10.0 * time.delta_seconds();
                        target_transform.translation.z -=
                            rotation_y.cos() * 10.0 * time.delta_seconds();

                        target_transform.rotation = Quat::from_rotation_y(rotation_y);
                    }
                }
            }
            CharacterControllerEvent::MoveBackward => {
                for camera in camera_query.iter() {
                    if let Ok(mut target_transform) = target_query.get_mut(camera.target) {
                        let rotation_y = camera.rotate_y;
                        target_transform.translation.x +=
                            rotation_y.sin() * 10.0 * time.delta_seconds();
                        target_transform.translation.z +=
                            rotation_y.cos() * 10.0 * time.delta_seconds();

                        target_transform.rotation = Quat::from_rotation_y(rotation_y);
                    }
                }
            }
            CharacterControllerEvent::MoveLeft => {
                for camera in camera_query.iter() {
                    if let Ok(mut target_transform) = target_query.get_mut(camera.target) {
                        let rotation_y = camera.rotate_y;
                        target_transform.translation.x -=
                            rotation_y.cos() * 10.0 * time.delta_seconds();
                        target_transform.translation.z +=
                            rotation_y.sin() * 10.0 * time.delta_seconds();

                        target_transform.rotation = Quat::from_rotation_y(rotation_y);
                    }
                }
            }
            CharacterControllerEvent::MoveRight => {
                for camera in camera_query.iter() {
                    if let Ok(mut target_transform) = target_query.get_mut(camera.target) {
                        let rotation_y = camera.rotate_y;
                        target_transform.translation.x +=
                            rotation_y.cos() * 10.0 * time.delta_seconds();
                        target_transform.translation.z -=
                            rotation_y.sin() * 10.0 * time.delta_seconds();

                        target_transform.rotation = Quat::from_rotation_y(rotation_y);
                    }
                }
            }
            _ => (),
        }
    }
}

pub struct ThirdPartyControllerPlugin;
impl Plugin for ThirdPartyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            third_party_camera::ThirdPartyCameraPlugin,
            input::InputMappingPlugin::<CharacterControllerEvent>::default(),
        ))
        .insert_resource(default_character_controller_event_mapping())
        .add_systems(Update, third_party_camera_controller_system);
    }
}
