use bevy::prelude::*;
use bevy_helper_tools::despawn;

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    Start,
    End,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(despawn::CleanupPlugin(GameState::Start))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, switch_state)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle::default(),
        despawn::Cleanup(GameState::Start),
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        despawn::Cleanup(GameState::Start),
    ));
}

fn switch_state(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::End);
    }
}
