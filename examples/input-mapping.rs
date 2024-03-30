use bevy::prelude::*;
use some_bevy_tools::input::UserButtonInput::*;
use some_bevy_tools::input::{self, InputMapping};

#[derive(Debug, Default, Component)]
struct Duck;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(input::InputMappingPlugin::<AppAction>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, action_handler)
        .run();
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum AppAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Exit,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Duck,
    ));
    let input_mapping: InputMapping<AppAction> = [
        (KeyPressed(KeyCode::ArrowUp), AppAction::MoveUp),
        (KeyPressed(KeyCode::KeyW), AppAction::MoveUp),
        (KeyPressed(KeyCode::ArrowDown), AppAction::MoveDown),
        (KeyPressed(KeyCode::KeyS), AppAction::MoveDown),
        (KeyPressed(KeyCode::ArrowLeft), AppAction::MoveLeft),
        (KeyPressed(KeyCode::KeyA), AppAction::MoveLeft),
        (KeyPressed(KeyCode::ArrowRight), AppAction::MoveRight),
        (KeyPressed(KeyCode::KeyD), AppAction::MoveRight),
        (KeyPressed(KeyCode::Escape), AppAction::Exit),
    ]
    .into();
    commands.insert_resource(input_mapping);
}

fn action_handler(
    mut actions: EventReader<input::ActionEvent<AppAction>>,
    mut duck_query: Query<&mut Transform, With<Duck>>,
) {
    for action in actions.read() {
        match action.action {
            AppAction::MoveUp => {
                for mut duck_transform in duck_query.iter_mut() {
                    duck_transform.translation.y += 10.0;
                }
            }
            AppAction::MoveDown => {
                for mut duck_transform in duck_query.iter_mut() {
                    duck_transform.translation.y -= 10.0;
                }
            }
            AppAction::MoveLeft => {
                for mut duck_transform in duck_query.iter_mut() {
                    duck_transform.translation.x -= 10.0;
                }
            }
            AppAction::MoveRight => {
                for mut duck_transform in duck_query.iter_mut() {
                    duck_transform.translation.x += 10.0;
                }
            }
            AppAction::Exit => {
                std::process::exit(0);
            }
        }
    }
}
