//! Module for detecting collisions between two entities.
//!
//! It will automatically produce events which can be used to listen for collisions.  It will also order the
//! Entities in the event in the order they are specifified in type parameters.
//!
//! # Examples
//! ```rust
//! use bevy_helper_tools::collision_detection;
//! use bevy::prelude::*;
//!
//! #[derive(Debug, Component, Default)]
//! struct Duck;
//!
//! #[derive(Debug, Component, Default)]
//! struct OtherDuck;
//!
//! fn main() {
//!     App::new()
//!     //    .add_plugins(DefaultPlugins)
//!         .add_plugins(collision_detection::CollisionDetectionPlugin::<
//!             Duck,
//!             OtherDuck,
//!         >::default())
//!         .add_systems(Update, check_collision);
//!     //    .run();
//! }
//!
//! fn check_collision(
//!     mut collision_events: EventReader<collision_detection::CollisionEventStart<Duck, OtherDuck>>,
//! ) {
//!     for collision_detection::CollisionEventStart(duck_entity, other_duck_entity, _) in
//!         collision_events.read() {
//!         println!("{:?} collided with {:?}", duck_entity, other_duck_entity);
//!     }
//! }
//! ```

use bevy::prelude::*;
#[cfg(feature = "bevy_rapier2d")]
use bevy_rapier2d::prelude::*;
#[cfg(feature = "bevy_rapier3d")]
use bevy_rapier2d::prelude::*;

//#[derive(Debug, Component)]
//pub struct CollisionDetection<C1, C2>
//where
//    C1: Component,
//    C2: Component,
//{
//    _c1: std::marker::PhantomData<C1>,
//    _c2: std::marker::PhantomData<C2>,
//}

/// Event that is triggered when a collision is detected between two entities.
///
/// It will be triggered when the collision starts.
#[derive(Debug, Event)]
pub struct CollisionEventStart<C1: Component, C2: Component>(
    pub Entity,
    pub Entity,
    pub std::marker::PhantomData<(C1, C2)>,
);

/// Event that is triggered when a collision is detected between two entities
///
/// It will be triggered when the collision stops.
#[derive(Debug, Event)]
pub struct CollisionEventStop<C1: Component, C2: Component>(
    pub Entity,
    pub Entity,
    pub std::marker::PhantomData<(C1, C2)>,
);

/// A system which checks for collisions between two specific components.
///
/// It will produce CollisionEventStart and CollisionEventStop events when a collision is detected.
pub fn collision_detection_system<C1: Component, C2: Component>(
    mut collision_event_start_writer: EventWriter<CollisionEventStart<C1, C2>>,
    mut collision_event_stop_writer: EventWriter<CollisionEventStop<C1, C2>>,
    mut collision_events: EventReader<CollisionEvent>,
    c1_query: Query<Entity, With<C1>>,
    c2_query: Query<Entity, With<C2>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if let (Ok(c1), Ok(c2)) = (c1_query.get(*entity1), c2_query.get(*entity2)) {
                    collision_event_start_writer.send(CollisionEventStart(
                        c1,
                        c2,
                        std::marker::PhantomData,
                    ));
                } else if let (Ok(c1), Ok(c2)) = (c1_query.get(*entity2), c2_query.get(*entity1)) {
                    collision_event_start_writer.send(CollisionEventStart(
                        c1,
                        c2,
                        std::marker::PhantomData,
                    ));
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if let (Ok(c1), Ok(c2)) = (c1_query.get(*entity1), c2_query.get(*entity2)) {
                    collision_event_stop_writer.send(CollisionEventStop(
                        c1,
                        c2,
                        std::marker::PhantomData,
                    ));
                } else if let (Ok(c1), Ok(c2)) = (c1_query.get(*entity2), c2_query.get(*entity1)) {
                    collision_event_stop_writer.send(CollisionEventStop(
                        c1,
                        c2,
                        std::marker::PhantomData,
                    ));
                }
            }
        }
    }
}

/// Easy to use collision detection of two specific components.
///
/// It will produce CollisionEventStart and CollisionEventStop events when a collision is detected.
#[derive(Default)]
pub struct CollisionDetectionPlugin<C1: Component, C2: Component> {
    _c1: std::marker::PhantomData<C1>,
    _c2: std::marker::PhantomData<C2>,
}
impl<C1: Component, C2: Component> Plugin for CollisionDetectionPlugin<C1, C2> {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEventStart<C1, C2>>()
            .add_event::<CollisionEventStop<C1, C2>>()
            .add_systems(Update, collision_detection_system::<C1, C2>);
    }
}
