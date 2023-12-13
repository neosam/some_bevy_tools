//! Provides a range type as components.
//!
//! Ranges can be used to define a floating point between two values.
//! This can for example be used to define the health of an entity,
//! its stamina, or its mana.
//!
//! It will also automatically modify the range based on the change_per_second
//! attribute to automatically fill up mana or heal.
//!
//! When the range reaches a limit, an event will be emitted.  The events
//! are StartRangeLimitReachedEvent and EndRangeLimitReachedEvent.  They can
//! be used to trigger specific actions like dying.
//!
//! # Examples
//! ```rust
//! use bevy_helper_tools::range::{Range, RangePlugin};
//! use bevy::prelude::*;
//!
//! #[derive(Default)]
//! struct Health;
//!
//! App::new()
//!     .add_plugins(RangePlugin::<Health>::default());
use bevy::prelude::*;

/// Quantize `value` to the nearest multiple of `step`.
///
/// # Examples
/// ```rust
/// use bevy_helper_tools::range::quantize;
///
/// assert_eq!(quantize(0.0, 1.0), 0.0);
/// assert_eq!(quantize(1.0, 1.0), 1.0);
/// assert_eq!(quantize(1.1, 1.0), 1.0);
/// assert_eq!(quantize(1.5, 1.0), 2.0);
/// assert_eq!(quantize(1.9, 1.0), 2.0);
/// assert_eq!(quantize(2.0, 1.0), 2.0);
/// assert_eq!(quantize(1.4, 0.5), 1.5);
/// assert_eq!(quantize(1.6, 0.5), 1.5);
/// ```
pub fn quantize(value: f32, step: f32) -> f32 {
    (value / step).round() * step
}

/// The result of modifying a range.  It is Ok if the value is within the range,
/// StartLimitReached if the value is below the range, and EndLimitReached if the
/// value is above the range.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ModifyRangeResult {
    Ok,
    StartLimitReached { low_limit: f32, value: f32 },
    EndLimitReached { high_limit: f32, value: f32 },
}

/// A range that contains a floating point between two values.
///
/// It uses a type parameter to allow for different
/// types of ranges.  For example, it could be used to control the health of
/// an entity.  Another type parameter could be used to control the speed of
/// an entity or stamina.
///
/// The attribute change_per_second can be used to automatically modify the
/// current value of the range.
///
/// # Examples
/// ```rust
/// use bevy_helper_tools::range::Range;
///
/// #[derive(Default)]
/// struct Health;
///
/// let mut range = Range::<Health>::default()
///     .with_start(0.0)
///     .with_end(10.0)
///     .with_current(5.0)
///     .with_quantize(0.5);
/// assert_eq!(range.get(), 5.0);
/// range.set(6.0);
/// assert_eq!(range.get(), 6.0);
/// ```
#[derive(Clone, Debug, Component, PartialEq)]
pub struct Range<T> {
    start: f32,
    end: f32,
    current: f32,
    quantize: f32,
    change_per_second: f32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Range<T> {
    /// Create a new range.
    /// The current value is set to the end value and quantize is set to 1.
    /// The change_per_second is set to 0 and so the value will not be modified automatically.
    pub fn new(start: f32, end: f32) -> Range<T> {
        Range {
            start,
            end,
            current: end,
            quantize: 1.0,
            change_per_second: 0.0,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new range with the current value set to the start value.
    pub fn with_start(self, start: f32) -> Range<T> {
        Range { start, ..self }
    }
    /// Create a new range with the current value set to the end value.
    pub fn with_end(self, end: f32) -> Range<T> {
        Range { end, ..self }
    }
    /// Create a new range with the current value set to the current value.
    pub fn with_current(self, current: f32) -> Range<T> {
        Range { current, ..self }
    }
    /// Create a new range with the quantize value set to the quantize value.
    pub fn with_quantize(self, quantize: f32) -> Range<T> {
        Range { quantize, ..self }
    }
    /// Create a new range with the change_per_second value set to the change_per_second value.
    pub fn with_change_per_second(self, change_per_second: f32) -> Range<T> {
        Range {
            change_per_second,
            ..self
        }
    }

    /// Set the quantize value.
    pub fn set_quantize(&mut self, quantize: f32) {
        self.quantize = quantize
    }
    /// Get the quantize value.
    pub fn get_quantize(&self) -> f32 {
        self.quantize
    }
    /// Set the start value.
    pub fn get_start(&self) -> f32 {
        self.start
    }
    /// Set the end value.
    pub fn get_end(&self) -> f32 {
        self.end
    }
    /// Set the current value.
    pub fn set_change_per_second(&mut self, change_per_second: f32) {
        self.change_per_second = change_per_second
    }
    /// Get the current value.
    pub fn get_change_per_second(&self) -> f32 {
        self.change_per_second
    }

    /// Set the current value.
    /// The result is Ok if the value is within the range,
    /// StartLimitReached if the value is equal or below the range, and
    /// EndLimitReached if the value is equal or above the range.
    /// If a value is outside the range, the current value is set to the
    /// closest limit.
    pub fn set(&mut self, value: f32) -> ModifyRangeResult {
        self.current = value;
        if self.current <= self.start {
            self.current = self.start;
            ModifyRangeResult::StartLimitReached {
                low_limit: self.start,
                value,
            }
        } else if self.current >= self.end {
            self.current = self.end;
            ModifyRangeResult::EndLimitReached {
                high_limit: self.end,
                value,
            }
        } else {
            ModifyRangeResult::Ok
        }
    }

    /// Get the current value.
    /// The value will be quantized to the quantize value.
    pub fn get(&self) -> f32 {
        quantize(self.current, self.quantize)
    }

    /// Modify the current value.
    /// The result is Ok if the value is within the range,
    /// StartLimitReached if the value is equal or below the range, and
    /// EndLimitReached if the value is equal or above the range.
    /// If a value is outside the range, the current value is set to the
    /// closest limit.
    pub fn modify(&mut self, delta: f32) -> ModifyRangeResult {
        self.set(self.current + delta)
    }
}

impl<T> Default for Range<T> {
    fn default() -> Self {
        Range::new(0.0, 1.0)
    }
}

/// An event which is sent when a range hits a start limit.
///
/// An example usage could be a health range which would mean the entity dies.
#[derive(Debug, Event)]
pub struct StartRangeLimitReachedEvent<T> {
    pub entity: Entity,
    _phantom: std::marker::PhantomData<T>,
}

/// An event which is sent when a range hits an end limit.
///
/// For example for stamina it could mean that the stamina is full again.
#[derive(Debug, Event)]
pub struct EndRangeLimitReachedEvent<T> {
    pub entity: Entity,
    _phantom: std::marker::PhantomData<T>,
}

/// A system to update the range values based on their change_per_second attribute.
pub fn update_range<T: Send + Sync + 'static>(
    mut range_query: Query<(Entity, &mut Range<T>)>,
    time: Res<Time>,
    mut start_range_limit_reached_event_writer: EventWriter<StartRangeLimitReachedEvent<T>>,
    mut end_range_limit_reached_event_writer: EventWriter<EndRangeLimitReachedEvent<T>>,
) {
    for (entity, mut range) in range_query.iter_mut() {
        let change_per_second = range.get_change_per_second();
        match range.modify(change_per_second * time.delta_seconds()) {
            ModifyRangeResult::Ok => {}
            ModifyRangeResult::StartLimitReached { .. } => {
                start_range_limit_reached_event_writer.send(StartRangeLimitReachedEvent {
                    entity,
                    _phantom: std::marker::PhantomData,
                });
            }
            ModifyRangeResult::EndLimitReached { .. } => {
                end_range_limit_reached_event_writer.send(EndRangeLimitReachedEvent {
                    entity,
                    _phantom: std::marker::PhantomData,
                })
            }
        }
    }
}

/// Add support for `Range`s in your game.
///
/// It will produce StartRangeLimitReachedEvent and EndRangeLimitReachedEvent when the
/// range reaches a limit.  It will also update the Range component based on the change_per_second
/// attribute.
#[derive(Debug, Default)]
pub struct RangePlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}
impl<T: Send + Sync + 'static> Plugin for RangePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<StartRangeLimitReachedEvent<T>>()
            .add_event::<EndRangeLimitReachedEvent<T>>()
            .add_systems(Update, update_range::<T>);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Health;
    type HealthRange = Range<Health>;

    #[test]
    fn test_range_basic_getter_and_setter() {
        let mut range = HealthRange::new(0.0, 10.0);
        assert_eq!(range.get(), 10.0);
        range.set(5.0);
        assert_eq!(range.get(), 5.0);
        range.set(11.0);
        assert_eq!(range.get(), 10.0);
        range.set(-1.0);
        assert_eq!(range.get(), 0.0);
    }

    #[test]
    fn test_range_set_responses() {
        let mut range = HealthRange::new(0.0, 10.0);
        assert_eq!(range.set(5.0), ModifyRangeResult::Ok);
        assert_eq!(range.get(), 5.0);
        assert_eq!(
            range.set(11.0),
            ModifyRangeResult::EndLimitReached {
                high_limit: 10.0,
                value: 11.0
            }
        );
        assert_eq!(range.get(), 10.0);
        assert_eq!(
            range.set(-1.0),
            ModifyRangeResult::StartLimitReached {
                low_limit: 0.0,
                value: -1.0
            }
        );
        assert_eq!(range.get(), 0.0);

        range.set(9.0);
        assert_eq!(
            range.modify(2.0),
            ModifyRangeResult::EndLimitReached {
                high_limit: 10.0,
                value: 11.0
            }
        );

        range.set(1.0);
        assert_eq!(
            range.modify(-2.0),
            ModifyRangeResult::StartLimitReached {
                low_limit: 0.0,
                value: -1.0
            }
        );

        assert_eq!(
            range.set(0.0),
            ModifyRangeResult::StartLimitReached {
                low_limit: 0.0,
                value: 0.0
            }
        );

        assert_eq!(
            range.set(10.0),
            ModifyRangeResult::EndLimitReached {
                high_limit: 10.0,
                value: 10.0
            }
        )
    }

    #[test]
    fn test_range_quantize() {
        let mut range = HealthRange::new(0.0, 10.0);
        range.set(5.4);
        assert_eq!(range.get(), 5.0);

        range.set_quantize(0.5);
        range.set(5.6);
        assert_eq!(range.get(), 5.5);
        range.set(5.4);
        assert_eq!(range.get(), 5.5);
    }

    #[test]
    fn test_range_builder() {
        let range = HealthRange::default()
            .with_start(1.0)
            .with_end(42.0)
            .with_current(23.0)
            .with_quantize(0.5)
            .with_change_per_second(1.5);

        assert_eq!(range.get_start(), 1.0);
        assert_eq!(range.get_end(), 42.0);
        assert_eq!(range.get(), 23.0);
        assert_eq!(range.get_quantize(), 0.5);
        assert_eq!(range.get_change_per_second(), 1.5);
    }
}
