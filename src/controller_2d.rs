//! Define some basic actions and keymapping to get started with
//! 2D games quickly.

use crate::input::{self, InputMapping, UserInput::*};
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
