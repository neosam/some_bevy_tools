//! User input handling.
//!
//! Maps use inputs to events.
//!
//! ## Example
//! ```rust
//! use bevy::prelude::*;
//! use some_bevy_tools::input;
//! use some_bevy_tools::input::UserButtonInput::*;
//!
//! #[derive(Clone, Eq, PartialEq, Hash)]
//! enum AppAction {
//!     Exit,
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.insert_resource::<input::InputMapping<AppAction>>(
//!         [
//!             (KeyPressed(KeyCode::Escape), AppAction::Exit),
//!         ].into()
//!     );
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

use bevy::{
    input::mouse::{self, MouseWheel},
    prelude::*,
    utils::hashbrown::HashSet,
};
use std::hash::Hash;

#[derive(Resource)]
pub struct InputMapping<Action: PartialEq> {
    button_mapping: Vec<ButtonMappingItem<Action>>,
    slider_mapping: Vec<DirectionalSliderMappingItem<Action>>,
}
impl<Action: Eq> InputMapping<Action> {
    pub fn add_button_mapping(&mut self, item: ButtonMappingItem<Action>) {
        self.button_mapping.push(item)
    }
    pub fn remove_button_mapping(&mut self, item: &ButtonMappingItem<Action>) {
        self.button_mapping.retain(|i| i != item)
    }

    pub fn add_directional_mapping(&mut self, item: DirectionalSliderMappingItem<Action>) {
        self.slider_mapping.push(item)
    }
    pub fn remove_directional_mapping(&mut self, item: &DirectionalSliderMappingItem<Action>) {
        self.slider_mapping.retain(|i| {
            i.action != item.action || i.slider_mapping_type != item.slider_mapping_type
        })
    }

    pub fn get_mappings_as_slice(&self) -> &[ButtonMappingItem<Action>] {
        &self.button_mapping
    }
    pub fn get_directional_mappings_as_slice(&self) -> &[DirectionalSliderMappingItem<Action>] {
        &self.slider_mapping
    }
}

impl<Action: Clone + PartialEq, const N: usize> From<[(UserButtonInput, Action); N]>
    for InputMapping<Action>
{
    fn from(item: [(UserButtonInput, Action); N]) -> Self {
        Self {
            button_mapping: item.iter().cloned().map(Into::into).collect(),
            slider_mapping: Vec::new(),
        }
    }
}

impl<Action: Clone + PartialEq, const N: usize, const M: usize>
    From<(
        [(UserButtonInput, Action); N],
        [(SliderMappingType, Action, f32); M],
    )> for InputMapping<Action>
{
    fn from(
        item: (
            [(UserButtonInput, Action); N],
            [(SliderMappingType, Action, f32); M],
        ),
    ) -> Self {
        Self {
            button_mapping: item.0.iter().cloned().map(Into::into).collect(),
            slider_mapping: item.1.iter().cloned().map(Into::into).collect(),
        }
    }
}
impl<Action: Clone + PartialEq, const N: usize, const M: usize>
    From<(
        [(UserButtonInput, Action); N],
        [(SliderMappingType, Action, f32, f32); M],
    )> for InputMapping<Action>
{
    fn from(
        item: (
            [(UserButtonInput, Action); N],
            [(SliderMappingType, Action, f32, f32); M],
        ),
    ) -> Self {
        Self {
            button_mapping: item.0.iter().cloned().map(Into::into).collect(),
            slider_mapping: item.1.iter().cloned().map(Into::into).collect(),
        }
    }
}

/// Maps a user input to a specific action.
#[derive(PartialEq)]
pub struct ButtonMappingItem<Action: PartialEq> {
    pub input: UserButtonInput,
    pub action: Action,
}

impl<Action: PartialEq> From<(UserButtonInput, Action)> for ButtonMappingItem<Action> {
    fn from(item: (UserButtonInput, Action)) -> Self {
        Self {
            input: item.0,
            action: item.1,
        }
    }
}

/// Input types which are either on or off.
///
/// This can be a key on a keyboard, mouse wheel, mouse button, or controller button.
///
/// The name is a bit weird but ButtonInput shadows a type from Bevy and I want to prevent that.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UserButtonInput {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    KeyPressed(KeyCode),
    MouseScrollUp,
    MouseScrollDown,
}

#[derive(Clone, PartialEq)]
pub struct DirectionalSliderMappingItem<Action> {
    pub slider_mapping_type: SliderMappingType,
    pub action: Action,
    pub factor_x: f32,
    pub factor_y: f32,
}

/// Kind of user input which gives a value between 0.0 and 1.0 in the x and y direction.
///
/// This is usually a joystick or mouse movement.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
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
    input: Res<bevy::prelude::ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut motion_events: EventReader<mouse::MouseMotion>,
    mut mapping: ResMut<InputMapping<Action>>,
    mut key_event_writer: EventWriter<ActionEvent<Action>>,
    mut direction_slider_event_writer: EventWriter<DirectionSliderEvent<Action>>,
    mut actions: Local<HashSet<Action>>,
) {
    let mut scroll_up = false;
    let mut scroll_down = false;

    for scroll_event in scroll_events.read() {
        if scroll_event.y < 0.0 {
            scroll_up = true;
        } else if scroll_event.y > 0.0 {
            scroll_down = true;
        }
    }

    for item in mapping.button_mapping.iter_mut() {
        match item.input {
            UserButtonInput::KeyDown(key) if input.just_pressed(key) => {
                actions.insert(item.action.clone());
            }
            UserButtonInput::KeyUp(key) if input.just_released(key) => {
                actions.insert(item.action.clone());
            }
            UserButtonInput::KeyPressed(key) if input.pressed(key) => {
                actions.insert(item.action.clone());
            }
            UserButtonInput::MouseScrollUp if scroll_up => {
                actions.insert(item.action.clone());
            }
            UserButtonInput::MouseScrollDown if scroll_down => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_adding_and_removing_mappings() {
        let mut mapping: InputMapping<i32> = (
            [
                (UserButtonInput::KeyDown(KeyCode::KeyA), 1),
                (UserButtonInput::KeyUp(KeyCode::KeyA), 2),
            ],
            [(SliderMappingType::MouseMove(0.5), 3, 0.5, 0.5)],
        )
            .into();
        assert_eq!(2, mapping.get_mappings_as_slice().len());
        assert_eq!(1, mapping.get_directional_mappings_as_slice().len());

        mapping.add_button_mapping((UserButtonInput::KeyPressed(KeyCode::ArrowUp), 3).into());
        assert_eq!(3, mapping.get_mappings_as_slice().len());
        assert_eq!(1, mapping.get_directional_mappings_as_slice().len());

        mapping.add_directional_mapping((SliderMappingType::MouseMove(0.5), 4, 0.5, 0.5).into());
        assert_eq!(3, mapping.get_mappings_as_slice().len());
        assert_eq!(2, mapping.get_directional_mappings_as_slice().len());

        mapping.remove_button_mapping(&(UserButtonInput::KeyDown(KeyCode::KeyA), 1).into());
        assert_eq!(2, mapping.get_mappings_as_slice().len());
        assert_eq!(2, mapping.get_directional_mappings_as_slice().len());

        mapping
            .remove_directional_mapping(&(SliderMappingType::MouseMove(0.5), 3, 0.5, 0.5).into());
        assert_eq!(2, mapping.get_mappings_as_slice().len());
        assert_eq!(1, mapping.get_directional_mappings_as_slice().len());

        mapping
            .remove_directional_mapping(&(SliderMappingType::MouseMove(0.5), 4, 0.1, 0.1).into());
        assert_eq!(2, mapping.get_mappings_as_slice().len());
        assert_eq!(0, mapping.get_directional_mappings_as_slice().len());

        mapping.remove_button_mapping(&(UserButtonInput::KeyDown(KeyCode::KeyZ), 1).into());
        assert_eq!(2, mapping.get_mappings_as_slice().len());
        assert_eq!(0, mapping.get_directional_mappings_as_slice().len());

        mapping.remove_button_mapping(&(UserButtonInput::KeyUp(KeyCode::KeyA), 1).into());
        assert_eq!(2, mapping.get_mappings_as_slice().len());
        assert_eq!(0, mapping.get_directional_mappings_as_slice().len());

        mapping.remove_button_mapping(&(UserButtonInput::KeyUp(KeyCode::KeyA), 2).into());
        assert_eq!(1, mapping.get_mappings_as_slice().len());
        assert_eq!(0, mapping.get_directional_mappings_as_slice().len());
    }
}
