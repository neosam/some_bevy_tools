use bevy::prelude::*;
use some_bevy_tools::camera_2d::Camera2DMode;
use some_bevy_tools::camera_2d::{Camera2DController, Camera2DPlugin};
use some_bevy_tools::controller_2d::{self, TopDownAction};
use some_bevy_tools::input;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Camera2DPlugin)
        .add_plugins(controller_2d::TopDownControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (action_handler, look_at_other_duck_system))
        .run();
}

#[derive(Component)]
pub struct Duck1;
#[derive(Component)]
pub struct Duck2;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let duck = commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("ducky.png"),
                transform: Transform::from_xyz(-300.0, 0.0, 0.0),
                ..Default::default()
            },
            Duck1,
        ))
        .id();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            ..Default::default()
        },
        Duck2,
    ));

    commands.spawn((
        Camera2dBundle::default(),
        Camera2DController::new_follow_with_speed(duck, 300.0),
    ));
}

fn action_handler(
    mut actions: EventReader<input::ActionEvent<TopDownAction>>,
    mut duck_query: Query<&mut Transform, With<Duck1>>,
) {
    for action in actions.read() {
        match action.action {
            TopDownAction::MoveUp => {
                for mut duck_transform in duck_query.iter_mut() {
                    duck_transform.translation.y += 10.0;
                }
            }
            TopDownAction::MoveDown => {
                for mut duck_transform in duck_query.iter_mut() {
                    duck_transform.translation.y -= 10.0;
                }
            }
            TopDownAction::MoveLeft => {
                for mut duck_transform in duck_query.iter_mut() {
                    duck_transform.translation.x -= 10.0;
                }
            }
            TopDownAction::MoveRight => {
                for mut duck_transform in duck_query.iter_mut() {
                    duck_transform.translation.x += 10.0;
                }
            }
            TopDownAction::Exit => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

#[derive(Default, PartialEq, Eq, Copy, Clone)]
enum CameraMovement {
    #[default]
    NoMovement,
    MoveToOtherDuck,
    MoveBack,
}

fn look_at_other_duck_system(
    original_duck_query: Query<Entity, With<Duck1>>,
    other_duck_query: Query<Entity, With<Duck2>>,
    mut actions: EventReader<input::ActionEvent<TopDownAction>>,
    mut camera_query: Query<&mut Camera2DController>,
    mut camera_movement: Local<CameraMovement>,
) {
    match *camera_movement {
        CameraMovement::NoMovement => {
            for action in actions.read() {
                match action.action {
                    TopDownAction::Action => {
                        let mut camera_controller = camera_query.single_mut();
                        camera_controller.target_entity = other_duck_query.single();
                        *camera_movement = CameraMovement::MoveToOtherDuck;
                        camera_controller.mode = Camera2DMode::Move;
                    }
                    _ => {}
                }
            }
        }
        CameraMovement::MoveToOtherDuck => {
            let mut camera_controller = camera_query.single_mut();
            if camera_controller.is_at_target {
                camera_controller.target_entity = original_duck_query.single();
                *camera_movement = CameraMovement::MoveBack;
            }
        }
        CameraMovement::MoveBack => {
            let mut camera_controller = camera_query.single_mut();
            if camera_controller.is_at_target {
                camera_controller.mode = Camera2DMode::Follow;
                *camera_movement = CameraMovement::NoMovement;
            }
        }
    }
}
