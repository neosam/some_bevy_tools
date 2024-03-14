//! User input handling.
//!
//! Maps use inputs to events.
//!
//! ## Example
//! ```rust
//! use bevy::prelude::*;
//! use some_bevy_tools::input;
//! use some_bevy_tools::input::UserInput::*;
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
//!      //.add_plugins(DefaultPlugins)
//!      .add_plugins(input::InputMappingPlugin::<AppAction>::default())
//!      .add_systems(Startup, setup)
//!      .add_systems(Update, action_handler);
//!      //.run();
//! ```

use bevy::{input::mouse, prelude::*, utils::hashbrown::HashSet};
use std::hash::Hash;

#[derive(Resource)]
pub struct InputMapping<Action> {
    mapping: Vec<InputMappingItem<Action>>,
    slider_mapping: Vec<DirectionalSliderMappingItem<Action>>,
}

impl<Action: Clone, const N: usize> From<[(UserInput, Action); N]> for InputMapping<Action> {
    fn from(item: [(UserInput, Action); N]) -> Self {
        Self {
            mapping: item.iter().cloned().map(Into::into).collect(),
            slider_mapping: Vec::new(),
        }
    }
}

impl<Action: Clone, const N: usize, const M: usize>
    From<(
        [(UserInput, Action); N],
        [(SliderMappingType, Action, f32); M],
    )> for InputMapping<Action>
{
    fn from(
        item: (
            [(UserInput, Action); N],
            [(SliderMappingType, Action, f32); M],
        ),
    ) -> Self {
        Self {
            mapping: item.0.iter().cloned().map(Into::into).collect(),
            slider_mapping: item.1.iter().cloned().map(Into::into).collect(),
        }
    }
}
impl<Action: Clone, const N: usize, const M: usize>
    From<(
        [(UserInput, Action); N],
        [(SliderMappingType, Action, f32, f32); M],
    )> for InputMapping<Action>
{
    fn from(
        item: (
            [(UserInput, Action); N],
            [(SliderMappingType, Action, f32, f32); M],
        ),
    ) -> Self {
        Self {
            mapping: item.0.iter().cloned().map(Into::into).collect(),
            slider_mapping: item.1.iter().cloned().map(Into::into).collect(),
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

#[derive(Clone)]
pub struct DirectionalSliderMappingItem<Action> {
    pub slider_mapping_type: SliderMappingType,
    pub action: Action,
    pub factor_x: f32,
    pub factor_y: f32,
}

#[derive(Clone)]
pub enum SliderMappingType {
    MouseMove(f32),
}

impl<Action> From<(SliderMappingType, Action, f32)> for DirectionalSliderMappingItem<Action> {
    fn from(item: (SliderMappingType, Action, f32)) -> Self {
        Self {
            slider_mapping_type: item.0,
            action: item.1,
            factor_x: item.2,
            factor_y: item.2,
        }
    }
}
impl<Action> From<(SliderMappingType, Action, f32, f32)> for DirectionalSliderMappingItem<Action> {
    fn from(item: (SliderMappingType, Action, f32, f32)) -> Self {
        Self {
            slider_mapping_type: item.0,
            action: item.1,
            factor_x: item.2,
            factor_y: item.3,
        }
    }
}

#[derive(Event)]
pub struct ActionEvent<Action> {
    pub action: Action,
}

#[derive(Event)]
pub struct DirectionSliderEvent<Action> {
    pub action: Action,
    pub x: f32,
    pub y: f32,
}

pub fn input_mapping_system<Action: Clone + Eq + Hash + Send + Sync + 'static>(
    input: Res<ButtonInput<KeyCode>>,
    mut motion_events: EventReader<mouse::MouseMotion>,
    mut mapping: ResMut<InputMapping<Action>>,
    mut key_event_writer: EventWriter<ActionEvent<Action>>,
    mut direction_slider_event_writer: EventWriter<DirectionSliderEvent<Action>>,
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
        key_event_writer.send(ActionEvent {
            action: action.clone(),
        });
    }
    actions.clear();

    if !mapping.slider_mapping.is_empty() {
        for event in motion_events.read() {
            for action in mapping.slider_mapping.iter() {
                direction_slider_event_writer.send(DirectionSliderEvent {
                    action: action.action.clone(),
                    x: event.delta.x * action.factor_x,
                    y: event.delta.y * action.factor_y,
                });
            }
        }
    }
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
            .add_event::<DirectionSliderEvent<Action>>()
            .add_systems(Update, input_mapping_system::<Action>);
    }
}
