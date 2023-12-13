use bevy::prelude::*;
use bevy_helper_tools::collision_detection;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Default, Component)]
struct Duck;

#[derive(Debug, Default, Component)]
struct OtherDuck;

fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(collision_detection::CollisionDetectionPlugin::<
            Duck,
            OtherDuck,
        >::default())
        .add_systems(Startup, setup)
        .add_systems(Update, check_collision)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(100.0, 100.0),
        RigidBody::Dynamic,
        Velocity::linear(Vec2::new(1.0, 0.0)),
        ActiveEvents::COLLISION_EVENTS,
        Duck,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            transform: Transform::from_xyz(-300.0, 0.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(100.0, 100.0),
        RigidBody::Dynamic,
        Velocity::linear(Vec2::new(100.0, 0.0)),
        ActiveEvents::COLLISION_EVENTS,
        OtherDuck,
    ));
}

fn check_collision(
    mut collision_events: EventReader<collision_detection::CollisionEventStart<Duck, OtherDuck>>,
) {
    for collision_detection::CollisionEventStart(duck_entity, other_duck_entity, _) in
        collision_events.read()
    {
        bevy::log::info!(
            "Collision detection: Duck = {:?} and OtherDuck = {:?}",
            duck_entity,
            other_duck_entity,
        );
    }
}
