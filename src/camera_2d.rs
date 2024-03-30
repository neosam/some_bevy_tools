//! Tools which helps with 2D cameras.
use bevy::prelude::*;

/// A 2D camera which automatically follows a target and allows to
/// move to move to a different target.
#[derive(Component)]
pub struct Camera2DController {
    pub speed: f32,
    pub allowed_distance: f32,
    pub mode: Camera2DMode,
    pub target_entity: Entity,
    pub is_at_target: bool,
}

impl Camera2DController {
    /// A camera which follows the target entity.
    pub fn new_follow_with_speed(target_entity: Entity, speed: f32) -> Self {
        Self {
            speed,
            allowed_distance: 100.0,
            mode: Camera2DMode::Follow,
            target_entity,
            is_at_target: true,
        }
    }
}

/// How the camera should behave.
pub enum Camera2DMode {
    /// Follows the target if the target is too far away. This is usual behavior
    /// in 2D games where the player can move in the center of the image whithout
    /// moveing the camera.  The camera only moves and follows if the player is
    /// is too far away from the center.
    Follow,

    /// Linear move to the target.
    Move,
}

/// System that handles the camera position.
///
/// At least the position of the entity which has the Camera2DController component.
pub fn camera_2d_controller_system(
    mut camera_query: Query<(&mut Transform, &mut Camera2DController)>,
    target_query: Query<&Transform, Without<Camera2DController>>,
    time: Res<Time>,
) {
    for (mut camera_transform, mut controller) in camera_query.iter_mut() {
        let target_transform = match target_query.get(controller.target_entity) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let mut movement_vector = target_transform.translation - camera_transform.translation;
        match controller.mode {
            Camera2DMode::Follow => {
                if movement_vector.x < -controller.allowed_distance {
                    camera_transform.translation.x =
                        target_transform.translation.x + controller.allowed_distance;
                }
                if movement_vector.x > controller.allowed_distance {
                    camera_transform.translation.x =
                        target_transform.translation.x - controller.allowed_distance;
                }
                if movement_vector.y < -controller.allowed_distance {
                    camera_transform.translation.y =
                        target_transform.translation.y + controller.allowed_distance;
                }
                if movement_vector.y > controller.allowed_distance {
                    camera_transform.translation.y =
                        target_transform.translation.y - controller.allowed_distance;
                }
                controller.is_at_target = true;
            }
            Camera2DMode::Move => {
                movement_vector.z = 0.0;
                if movement_vector.length() < controller.speed / (1.0 / time.delta_seconds()) {
                    camera_transform.translation.x = target_transform.translation.x;
                    camera_transform.translation.y = target_transform.translation.y;
                    controller.is_at_target = true;
                } else {
                    let normalized_movement_vector = movement_vector.normalize_or_zero();
                    let movement =
                        normalized_movement_vector * controller.speed * time.delta_seconds();
                    camera_transform.translation += movement;
                    controller.is_at_target = false;
                }
            }
        }
    }
}

/// Activate the Camera2D handling.
pub struct Camera2DPlugin;

impl Plugin for Camera2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, camera_2d_controller_system);
    }
}
