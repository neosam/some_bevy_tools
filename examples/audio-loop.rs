use bevy::prelude::*;
use some_bevy_tools::audio_loop::{AudioLoopEvent, LoopableAudioSource};
use some_bevy_tools::input::UserInput::*;
use some_bevy_tools::{audio_loop, input};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(input::InputMappingPlugin::<AppAction>::default())
        .add_plugins(audio_loop::AudioLoopPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum AppAction {
    NextPart,
    PrevPart,
}

#[derive(Resource)]
pub struct AudioHandles {
    pub audio_loop: Handle<LoopableAudioSource>,
}

#[derive(Resource)]
pub struct AudioLoopHandles {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_events: EventWriter<AudioLoopEvent>,
) {
    let audio_source: Handle<LoopableAudioSource> = asset_server.load("ehh-ehh.ogg");
    commands.insert_resource(AudioHandles {
        audio_loop: audio_source.clone(),
    });

    commands.spawn(AudioSourceBundle {
        source: audio_source.clone(),
        ..Default::default()
    });
    audio_events.send(AudioLoopEvent::EndPositionImmediate(7.38, audio_source));

    let input_mapping: input::InputMapping<AppAction> = [
        (KeyDown(KeyCode::Space), AppAction::NextPart),
        (KeyDown(KeyCode::ArrowRight), AppAction::NextPart),
        (KeyDown(KeyCode::ArrowLeft), AppAction::PrevPart),
    ]
    .into();
    commands.insert_resource(input_mapping);
}

fn update(
    mut input_mapping: EventReader<input::ActionEvent<AppAction>>,
    mut audio_events: EventWriter<AudioLoopEvent>,
    audio_handles: Res<AudioHandles>,
) {
    for event in input_mapping.read() {
        match event.action {
            AppAction::NextPart => {
                audio_events.send(AudioLoopEvent::LoopOffset(
                    7.38,
                    audio_handles.audio_loop.clone(),
                ));
            }
            AppAction::PrevPart => {
                audio_events.send(AudioLoopEvent::LoopOffset(
                    -7.38,
                    audio_handles.audio_loop.clone(),
                ));
            }
        }
    }
}
