use bevy::prelude::*;
use some_bevy_tools::split_screen;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(split_screen::SplitScreenPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
struct MoveCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), split_screen::LeftCamera));
    commands.spawn((
        Camera2dBundle::default(),
        split_screen::RightCamera,
        MoveCamera,
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("ducky.png"),
        ..Default::default()
    },));
}

fn update(time: Res<Time>, mut query: Query<&mut Transform, With<MoveCamera>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = 1000.0 * time.elapsed_seconds().sin();
    }
}
