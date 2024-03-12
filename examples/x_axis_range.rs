// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use some_bevy_tools::range;

static CHANGE_PER_SECOND: f32 = 50.0;

#[derive(Default)]
struct XAxis;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(range::RangePlugin::<XAxis>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (update_x_range_positions, detect_range_limit))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        range::Range::<XAxis>::default()
            .with_start(-100.0)
            .with_end(100.0)
            .with_change_per_second(-CHANGE_PER_SECOND),
    ));
}

fn update_x_range_positions(mut range_query: Query<(&mut Transform, &range::Range<XAxis>)>) {
    for (mut transform, range) in range_query.iter_mut() {
        transform.translation.x = range.get();
    }
}

fn detect_range_limit(
    mut range_query: Query<&mut range::Range<XAxis>>,
    mut start_range_limit_reached_event_reader: EventReader<
        range::StartRangeLimitReachedEvent<XAxis>,
    >,
    mut end_range_limit_reached_event_reader: EventReader<range::EndRangeLimitReachedEvent<XAxis>>,
) {
    for event in start_range_limit_reached_event_reader.read() {
        if let Ok(mut range) = range_query.get_mut(event.entity) {
            range.set_change_per_second(CHANGE_PER_SECOND);
        }
    }
    for event in end_range_limit_reached_event_reader.read() {
        if let Ok(mut range) = range_query.get_mut(event.entity) {
            range.set_change_per_second(-CHANGE_PER_SECOND);
        }
    }
}
