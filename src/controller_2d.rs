//! Define some basic actions and keymapping to get started with
//! 2D games quickly.

use crate::input::{self, InputMapping, UserButtonInput::*};
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum TopDownAction {
    /* User wants to go up */
    MoveUp,

    /* User wants to go down */
    MoveDown,

    /* User wants to go left */
    MoveLeft,

    /* User wants to go right */
    MoveRight,

    /* User presses the action button (usually space) */
    Action,

    /* User presses the secondary action button (usually enter) */
    Action2,

    /* User presses the exit button which is usually escape to exit the game or to open a menu */
    Exit,
}

pub fn setup_top_down_mapping(mut commands: Commands) {
    let input_mapping: InputMapping<TopDownAction> = [
        (KeyPressed(KeyCode::ArrowUp), TopDownAction::MoveUp),
        (KeyPressed(KeyCode::KeyW), TopDownAction::MoveUp),
        (KeyPressed(KeyCode::ArrowDown), TopDownAction::MoveDown),
        (KeyPressed(KeyCode::KeyS), TopDownAction::MoveDown),
        (KeyPressed(KeyCode::ArrowLeft), TopDownAction::MoveLeft),
        (KeyPressed(KeyCode::KeyA), TopDownAction::MoveLeft),
        (KeyPressed(KeyCode::ArrowRight), TopDownAction::MoveRight),
        (KeyPressed(KeyCode::KeyD), TopDownAction::MoveRight),
        (KeyPressed(KeyCode::Space), TopDownAction::Action),
        (KeyPressed(KeyCode::Enter), TopDownAction::Action2),
        (KeyPressed(KeyCode::Escape), TopDownAction::Exit),
    ]
    .into();
    commands.insert_resource(input_mapping);
}

pub struct TopDownControllerPlugin;

impl Plugin for TopDownControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(input::InputMappingPlugin::<TopDownAction>::default())
            .add_systems(Startup, setup_top_down_mapping);
    }
}

#[derive(Component)]
pub struct SimpleTopDownController {
    pub speed: f32,
    pub active: bool,
}

impl SimpleTopDownController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            active: true,
        }
    }
}

fn simple_top_down_controller(
    mut actions: EventReader<input::ActionEvent<TopDownAction>>,
    mut entity_query: Query<
        (&SimpleTopDownController, &mut Transform),
        With<SimpleTopDownController>,
    >,
) {
    for action in actions.read() {
        match action.action {
            TopDownAction::MoveUp => {
                for (controller, mut transform) in entity_query.iter_mut() {
                    if controller.active {
                        transform.translation.y += controller.speed;
                    }
                }
            }
            TopDownAction::MoveDown => {
                for (controller, mut transform) in entity_query.iter_mut() {
                    if controller.active {
                        transform.translation.y -= controller.speed;
                    }
                }
            }
            TopDownAction::MoveLeft => {
                for (controller, mut transform) in entity_query.iter_mut() {
                    if controller.active {
                        transform.translation.x -= controller.speed;
                    }
                }
            }
            TopDownAction::MoveRight => {
                for (controller, mut transform) in entity_query.iter_mut() {
                    if controller.active {
                        transform.translation.x += controller.speed;
                    }
                }
            }
            _ => {}
        }
    }
}

pub struct SimpleTopDownControllerPlugin;

impl Plugin for SimpleTopDownControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TopDownControllerPlugin)
            .add_systems(Update, simple_top_down_controller);
    }
}
