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
}

pub fn default_character_controller_event_mapping() -> input::InputMapping<CharacterControllerEvent>
{
    input::InputMapping::from((
        [
            (
                input::UserInput::MouseScrollDown,
                CharacterControllerEvent::IncreaseCameraDistance,
            ),
            (
                input::UserInput::MouseScrollUp,
                CharacterControllerEvent::DecreaseCameraDistance,
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
    bevy::log::info!("third_party-camera-controller-system");
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
