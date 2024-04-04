use bevy::prelude::*;
use some_bevy_tools::audio_loop::{AudioLoopEvent, LoopableAudioSource};
use some_bevy_tools::input::UserButtonInput::*;
use some_bevy_tools::{audio_loop, input};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Use action to user input mapping in this example
        .add_plugins(input::InputMappingPlugin::<AppAction>::default())
        // Load the AudioLoopPlugin to actually be able to loop audio
        .add_plugins(audio_loop::AudioLoopPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

/// Define the actions that can be triggered by user input
#[derive(Clone, Eq, PartialEq, Hash)]
enum AppAction {
    /// Play the next part of the audio loop when the current loop is finished
    NextPart,

    /// Play the previous part of the audio loop when the current loop is finished
    PrevPart,

    /// Jump to the next part and move the position as well
    NextPartImmediate,

    /// Jump to the previous part and move the position as well
    PrevPartImmediate,

    /// Play the whole song
    PlaySong,

    /// Play the first loop
    PlayFirstLoop,
}

/// Resource to store the audio handles in order to modify the loop
/// position.
#[derive(Resource)]
pub struct AudioHandles {
    pub audio_loop: Handle<LoopableAudioSource>,
}

#[derive(Resource)]
pub struct AudioLoopHandles {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // AudioLoopEvents are used to communicate with the AudioLoopPlugin in order to change the loop position.
    mut audio_events: EventWriter<AudioLoopEvent>,
) {
    // Load the music and use LoopableAudioSource as the asset type.
    let audio_source: Handle<LoopableAudioSource> = asset_server.load("ehh-ehh.ogg");
    commands.insert_resource(AudioHandles {
        audio_loop: audio_source.clone(),
    });

    commands.spawn(AudioSourceBundle {
        source: audio_source.clone(),
        ..Default::default()
    });

    // Set the loop ending at 7.38 seconds so it will loop the first 7.38 seconds of the music
    // instead of the whole music.
    // The asset will not be loaded at this point but the the event will be buffered while it is
    // loading. Once loaded it will set the loop ending.
    audio_events.send(AudioLoopEvent::EndPositionImmediate(7.38, audio_source));

    // Define the input mapping for the actions.  Space and right arrow will move the loop forward
    // and left arrow will move the loop backwards.
    let input_mapping: input::InputMapping<AppAction> = [
        (KeyDown(KeyCode::ArrowRight), AppAction::NextPart),
        (KeyDown(KeyCode::ArrowLeft), AppAction::PrevPart),
        (KeyDown(KeyCode::ArrowUp), AppAction::NextPartImmediate),
        (KeyDown(KeyCode::ArrowDown), AppAction::PrevPartImmediate),
        (KeyDown(KeyCode::KeyD), AppAction::NextPart),
        (KeyDown(KeyCode::KeyA), AppAction::PrevPart),
        (KeyDown(KeyCode::KeyW), AppAction::NextPartImmediate),
        (KeyDown(KeyCode::KeyS), AppAction::PrevPartImmediate),
        (KeyDown(KeyCode::Space), AppAction::PlaySong),
        (KeyDown(KeyCode::Enter), AppAction::PlayFirstLoop),
    ]
    .into();
    commands.insert_resource(input_mapping);
}

fn update(
    mut input_mapping: EventReader<input::ActionEvent<AppAction>>,
    // Use the AudioLoopEvent to communicate with the AudioLoopPlugin
    mut audio_events: EventWriter<AudioLoopEvent>,
    audio_handles: Res<AudioHandles>,
) {
    for event in input_mapping.read() {
        match event.action {
            AppAction::NextPart => {
                // Tell the AudioLoopPlugin to move the loop position 7.38 seconds forward.
                audio_events.send(AudioLoopEvent::LoopOffset(
                    7.38,
                    audio_handles.audio_loop.clone(),
                ));
            }
            AppAction::PrevPart => {
                // Tell the AudioLoopPlugin to move the loop position 7.38 seconds backwards.
                audio_events.send(AudioLoopEvent::LoopOffset(
                    -7.38,
                    audio_handles.audio_loop.clone(),
                ));
            }
            AppAction::NextPartImmediate => {
                // Tell the AudioLoopPlugin to move the loop position 7.38 seconds forward.
                audio_events.send(AudioLoopEvent::LoopOffsetImmediate(
                    7.38,
                    audio_handles.audio_loop.clone(),
                ));
            }
            AppAction::PrevPartImmediate => {
                // Tell the AudioLoopPlugin to move the loop position 7.38 seconds backwards.
                audio_events.send(AudioLoopEvent::LoopOffsetImmediate(
                    -7.38,
                    audio_handles.audio_loop.clone(),
                ));
            }
            AppAction::PlaySong => {
                audio_events.send(AudioLoopEvent::LoopPosition(
                    0.0,
                    f32::MAX,
                    0.0,
                    audio_handles.audio_loop.clone(),
                ));
            }
            AppAction::PlayFirstLoop => {
                audio_events.send(AudioLoopEvent::LoopPosition(
                    0.0,
                    7.38,
                    0.0,
                    audio_handles.audio_loop.clone(),
                ));
            }
        }
    }
}
