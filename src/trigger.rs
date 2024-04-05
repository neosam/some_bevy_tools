use bevy::prelude::*;

use crate::collision_detection;

#[derive(Component, Default)]
pub struct SingleTrigger;

pub fn remove_after_collision<Emitter: Component>(
    mut commands: Commands,
    mut collision_events: EventReader<
        collision_detection::CollisionEventStop<Emitter, SingleTrigger>,
    >,
    query: Query<Entity, With<SingleTrigger>>,
) {
    for collision_event in collision_events.read() {
        if let Ok(entity) = query.get(collision_event.1) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Default)]
pub struct PhysicsTriggerPlugin<Emitter: Component + Default, Trigger: Component + Default> {
    _marker: std::marker::PhantomData<(Emitter, Trigger)>,
}
impl<Emitter: Component + Default, Trigger: Component + Default> Plugin
    for PhysicsTriggerPlugin<Emitter, Trigger>
{
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<collision_detection::CollisionDetectionPlugin<Emitter, SingleTrigger>>()
        {
            app.add_plugins(collision_detection::CollisionDetectionPlugin::<
                Emitter,
                SingleTrigger,
            >::default())
            .add_systems(Update, remove_after_collision::<Emitter>);
        }
        app.add_plugins(collision_detection::CollisionDetectionPlugin::<
            Emitter,
            Trigger,
        >::default());
    }
}
