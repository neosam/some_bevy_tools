//! # Some bevy tools
//!
//! This repo contains extensions for the great [Bevy Engine](https://bevyengine.org/). My goal is to have a crate which
//! provides ECS stuff I regularly use in projects to save me time in future Bevy Game Jams.  This crate tries to
//! make the usage as simple as possible so the developer can focus on the main content of the game.
//!
//! Currently supported features are:
//!
//! * Automatic despawn after a period of time.
//! * Automatic despawn of components on a state change.
//! * Range component which keeps its value between a min and a max value and writes events
//!   if min or max was reached.  For example it can be used for health to detect death.
//! * Simplified processing of events on collisions in rapier.
//! * Mapping of user inputs to custom events. (currently only keyboard events are supported for now)
//! * Loading of assets on a loading state and storing them automatically in a resource using reflect.
//! * Split screen support.
//! * SBS support. It is basically a split screen which allows a sterioscopic view by using special
//!   hardware like XReal or Virture glasses.
//!
//! Additionally, I try to document each module with at least one example. This should ensure that
//! there are no accidential breaking changes.

#[cfg(feature = "audio_loop")]
pub mod audio_loop;
pub mod camera_2d;
#[cfg(feature = "bevy_rapier2d")]
pub mod collision_detection;
#[cfg(feature = "bevy_rapier3d")]
pub mod collision_detection;
pub mod controller_2d;
pub mod despawn;
pub mod input;
#[cfg(feature = "loading")]
pub mod loading;
#[cfg(feature = "bevy_rapier2d")]
pub mod physics2d;
pub mod range;
#[cfg(feature = "sbs_3d")]
pub mod sbs_3d;
#[cfg(feature = "split_screen")]
pub mod split_screen;
pub mod third_party_camera;
pub mod third_party_controller;
#[cfg(feature = "bevy_rapier2d")]
pub mod trigger;
#[cfg(feature = "bevy_rapier3d")]
pub mod trigger;
