use bevy::prelude::*;
use some_bevy_tools::audio_loop::LoopableAudioSource;
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let audio_source: Handle<LoopableAudioSource> = asset_server.load("ehh-ehh.ogg");
    commands.insert_resource(AudioHandles {
        audio_loop: audio_source.clone(),
    });

    commands.spawn(AudioSourceBundle {
        source: audio_source,
        ..Default::default()
    });

    let input_mapping: input::InputMapping<AppAction> = [
        (KeyDown(KeyCode::Space), AppAction::NextPart),
        (KeyDown(KeyCode::ArrowRight), AppAction::NextPart),
        (KeyDown(KeyCode::ArrowLeft), AppAction::PrevPart),
    ]
    .into();
    commands.insert_resource(input_mapping);
}

fn update(
    mut audio_loops: ResMut<Assets<LoopableAudioSource>>,
    mut input_mapping: EventReader<input::ActionEvent<AppAction>>,
    audio_handles: Res<AudioHandles>,
    asset_server: Res<AssetServer>,
    mut initialized: Local<bool>,
) {
    if asset_server.get_load_state(audio_handles.audio_loop.clone())
        == Some(bevy::asset::LoadState::Loaded)
        && *initialized == false
    {
        if let Some(audio_loop) = audio_loops.get_mut(audio_handles.audio_loop.clone()) {
            audio_loop.set_loop_end(7.38);
            *initialized = true;
        }
    }
    for event in input_mapping.read() {
        match event.action {
            AppAction::NextPart => {
                let audio_loop = audio_handles.audio_loop.clone();
                if let Some(audio_loop) = audio_loops.get_mut(audio_loop) {
                    audio_loop.add_loop_offset(7.38);
                }
            }
            AppAction::PrevPart => {
                let audio_loop = audio_handles.audio_loop.clone();
                if let Some(audio_loop) = audio_loops.get_mut(audio_loop) {
                    audio_loop.add_start_offset(-7.38);
                }
            }
        }
    }
}
