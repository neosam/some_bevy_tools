#[cfg(feature = "bevy_rapier2d")]
pub mod collision_detection;
#[cfg(feature = "bevy_rapier3d")]
pub mod collision_detection;
pub mod despawn;
pub mod range;
