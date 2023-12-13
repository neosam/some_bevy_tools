use bevy::prelude::*;

pub fn quantize(value: f32, step: f32) -> f32 {
    (value / step).round() * step
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ModifyRangeResult {
    Ok,
    StartLimitReached { low_limit: f32, value: f32 },
    EndLimitReached { high_limit: f32, value: f32 },
}

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

    pub fn with_start(self, start: f32) -> Range<T> {
        Range { start, ..self }
    }
    pub fn with_end(self, end: f32) -> Range<T> {
        Range { end, ..self }
    }
    pub fn with_current(self, current: f32) -> Range<T> {
        Range { current, ..self }
    }
    pub fn with_quantize(self, quantize: f32) -> Range<T> {
        Range { quantize, ..self }
    }
    pub fn with_change_per_second(self, change_per_second: f32) -> Range<T> {
        Range {
            change_per_second,
            ..self
        }
    }

    pub fn set_quantize(&mut self, quantize: f32) {
        self.quantize = quantize
    }
    pub fn get_quantize(&self) -> f32 {
        self.quantize
    }
    pub fn get_start(&self) -> f32 {
        self.start
    }
    pub fn get_end(&self) -> f32 {
        self.end
    }

    pub fn set_change_per_second(&mut self, change_per_second: f32) {
        self.change_per_second = change_per_second
    }
    pub fn get_change_per_second(&self) -> f32 {
        self.change_per_second
    }

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

    pub fn get(&self) -> f32 {
        quantize(self.current, self.quantize)
    }

    pub fn modify(&mut self, delta: f32) -> ModifyRangeResult {
        self.set(self.current + delta)
    }
}

impl<T> Default for Range<T> {
    fn default() -> Self {
        Range::new(0.0, 1.0)
    }
}

#[derive(Debug, Event)]
pub struct StartRangeLimitReachedEvent<T> {
    pub entity: Entity,
    _phantom: std::marker::PhantomData<T>,
}
#[derive(Debug, Event)]
pub struct EndRangeLimitReachedEvent<T> {
    pub entity: Entity,
    _phantom: std::marker::PhantomData<T>,
}

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
