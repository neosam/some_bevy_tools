//! Add health support for components.
//!
//! This is basically just a range which has type aliases.

use crate::range;
use bevy::prelude::*;

#[derive(Default)]
pub struct HealthMarker;

pub type Health = range::Range<HealthMarker>;
pub type DeathEvent = range::StartRangeLimitReachedEvent<HealthMarker>;
pub type FullHealEvent = range::EndRangeLimitReachedEvent<HealthMarker>;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(range::RangePlugin::<HealthMarker>::default());
    }
}
