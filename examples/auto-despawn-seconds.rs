use bevy::prelude::*;
use some_bevy_tools::despawn;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(despawn::AutoDespawnPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        despawn::AutoDespawn::with_duration(3.0),
    ));
}
