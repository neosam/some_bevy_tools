//! User input handling.
//!
//! Maps use inputs to events.
//!
//! ## Example
//! ```rust
//! use bevy::prelude::*;
//! use bevy_helper_tools::input;
//! use bevy_helper_tools::input::UserInput::*;
//!
//! #[derive(Clone, Eq, PartialEq, Hash)]
//! enum AppAction {
//!     Exit,
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.insert_resource(input::InputMapping::<AppAction>::from(
//!         [
//!             (KeyPressed(KeyCode::Escape), AppAction::Exit),
//!         ]
//!         .as_ref(),
//!     ));
//! }
//!
//! fn action_handler(
//!     mut actions: EventReader<input::ActionEvent<AppAction>>,
//! ) {
//!     for action in actions.read() {
//!         match action.action {
//!             AppAction::Exit => {
//!                 std::process::exit(0);
//!             }
//!         }
//!     }
//! }
//!
//!  App::new()
//!      .add_plugins(DefaultPlugins)
//!      .add_plugins(input::InputMappingPlugin::<AppAction>::default())
//!      .add_systems(Startup, setup)
//!      .add_systems(Update, action_handler);
//! ```

use bevy::{prelude::*, utils::hashbrown::HashSet};
use std::hash::Hash;

#[derive(Resource)]
pub struct InputMapping<Action> {
    mapping: Vec<InputMappingItem<Action>>,
}

impl<Action: Clone> From<&[(UserInput, Action)]> for InputMapping<Action> {
    fn from(item: &[(UserInput, Action)]) -> Self {
        Self {
            mapping: item.iter().cloned().map(Into::into).collect(),
        }
    }
}

/// Maps a user input to a specific action.
pub struct InputMappingItem<Action> {
    pub input: UserInput,
    pub action: Action,
}

impl<Action> From<(UserInput, Action)> for InputMappingItem<Action> {
    fn from(item: (UserInput, Action)) -> Self {
        Self {
            input: item.0,
            action: item.1,
        }
    }
}

#[derive(Clone)]
pub enum UserInput {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    KeyPressed(KeyCode),
}

#[derive(Event)]
pub struct ActionEvent<Action> {
    pub action: Action,
}

pub fn input_mapping_system<Action: Clone + Eq + Hash + Send + Sync + 'static>(
    input: Res<ButtonInput<KeyCode>>,
    mut mapping: ResMut<InputMapping<Action>>,
    mut writer: EventWriter<ActionEvent<Action>>,
    mut actions: Local<HashSet<Action>>,
) {
    for item in mapping.mapping.iter_mut() {
        match item.input {
            UserInput::KeyDown(key) if input.just_pressed(key) => {
                actions.insert(item.action.clone());
            }
            UserInput::KeyUp(key) if input.just_released(key) => {
                actions.insert(item.action.clone());
            }
            UserInput::KeyPressed(key) if input.pressed(key) => {
                actions.insert(item.action.clone());
            }
            _ => {}
        }
    }
    for action in actions.iter() {
        writer.send(ActionEvent {
            action: action.clone(),
        });
    }
    actions.clear();
}

pub struct InputMappingPlugin<Action> {
    __action: std::marker::PhantomData<Action>,
}

impl<Action> Default for InputMappingPlugin<Action> {
    fn default() -> Self {
        Self {
            __action: std::marker::PhantomData,
        }
    }
}

impl<Action: Clone + Eq + Hash + Send + Sync + 'static> Plugin for InputMappingPlugin<Action> {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent<Action>>()
            .add_systems(Update, input_mapping_system::<Action>);
    }
}
