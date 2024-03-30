//! Tools to support a third party camera.
//! 
//! The camera orbits a target entity. It can be rotated around the target entity and the distance can be changed.
use bevy::prelude::*;

#[derive(Component)]
pub struct ThirdPartyCamera {
    pub target: Entity,
    pub distance: f32,

    pub rotate_y: f32,
    pub rotate_x: f32,
}

pub fn third_party_camera_positioning(
    target_query: Query<&Transform, Without<ThirdPartyCamera>>,
    mut camera_query: Query<(&mut Transform, &ThirdPartyCamera)>,
) {
    for (mut camera_transform, camera) in camera_query.iter_mut() {
        if let Ok(target) = target_query.get(camera.target) {
            *camera_transform = calculate_camera_transform(
                target.translation,
                camera.distance,
                camera.rotate_y,
                camera.rotate_x,
            );
        }
    }
}

pub fn calculate_camera_transform(
    target_position: Vec3,
    distance: f32,
    rotate_y: f32,
    rotate_x: f32,
) -> Transform {
    Transform::from_translation(
        target_position + normalized_local_translation_vector(rotate_y, rotate_x) * distance,
    )
    .looking_at(target_position, Vec3::Y)
}

pub fn normalized_local_translation_vector(rotate_y: f32, rotate_x: f32) -> Vec3 {
    Vec3::new(
        rotate_y.sin() * rotate_x.cos(),
        rotate_x.sin(),
        rotate_y.cos() * rotate_x.cos(),
    )
}

pub struct ThirdPartyCameraPlugin;
impl Plugin for ThirdPartyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, third_party_camera_positioning);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn compare_f32(a: f32, b: f32) -> bool {
        let epsilon = 0.001;
        (a - b).abs() < epsilon
    }

    fn compare_vec3(a: Vec3, b: Vec3) -> bool {
        compare_f32(a.x, b.x) && compare_f32(a.y, b.y) && compare_f32(a.z, b.z)
    }

    fn assert_vec3(a: Vec3, b: Vec3) {
        assert!(
            compare_vec3(a, b),
            "Assert doesn't match:\n{:?}\n{:?}",
            a,
            b
        );
    }

    #[test]
    fn test_normalized_local_translation_vector() {
        assert_eq!(
            normalized_local_translation_vector(0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        assert_vec3(
            normalized_local_translation_vector(PI / 2.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        assert_vec3(
            normalized_local_translation_vector(PI, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        );
        assert_vec3(
            normalized_local_translation_vector(PI / 2.0 * 3.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
        );
        assert_vec3(
            normalized_local_translation_vector(PI * 2.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        assert_vec3(
            normalized_local_translation_vector(PI / 4.0, 0.0),
            Vec3::new(1.0 / 2.0_f32.sqrt(), 0.0, 1.0 / 2.0f32.sqrt()),
        );

        assert_vec3(
            normalized_local_translation_vector(0.0, PI / 2.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        assert_vec3(
            normalized_local_translation_vector(PI / 2.0, PI / 2.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        assert_vec3(
            normalized_local_translation_vector(0.0, -PI / 2.0),
            Vec3::new(0.0, -1.0, 0.0),
        );
        assert_vec3(
            normalized_local_translation_vector(PI / 4.0, PI / 4.0),
            Vec3::new(0.5, 1.0 / 2.0_f32.sqrt(), 0.5),
        );
    }
}
