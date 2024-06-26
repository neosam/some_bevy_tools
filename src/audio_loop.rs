//! Loop a music on a specific position.
//!
//! It introduces a new asset type called LoopableAudioSource which can be
//! loaded with the bevy asset server or the loading feature of this crate.
//!
//! ## Example
//! ```rust
//! use bevy::prelude::*;
//! use some_bevy_tools::audio_loop;
//!
//! #[derive(Resource)]
//! pub struct AudioHandles {
//!     pub audio_handle: Handle<audio_loop::LoopableAudioSource>,    
//! }
//!
//! fn startup(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
//!     let audio_source: Handle<audio_loop::LoopableAudioSource> = asset_server.load("ehh-ehh.ogg");
//!     commands.spawn(AudioSourceBundle {
//!         source: audio_source.clone(),
//!         ..Default::default()
//!     });
//!
//!     // Store the asset so we can change the loop points later.
//!     commands.insert_resource(AudioHandles { audio_handle: audio_source });
//! }
//!
//! fn update(
//!     mut audio_events: EventWriter<audio_loop::AudioLoopEvent>,
//!     audio_handles: Res<AudioHandles>,
//! ) {
//!     // Set the new start position which will be activated when the current end of the loop is reached.
//!     audio_events.send(audio_loop::AudioLoopEvent::StartPosition(7.38, audio_handles.audio_handle.clone()));
//!
//!     // Set the new end position which will be activated when the current end of the loop is reached.
//!     audio_events.send(audio_loop::AudioLoopEvent::EndPosition(7.38, audio_handles.audio_handle.clone()));
//!
//!     // Set the new start position which gets activated immediately.
//!     audio_events.send(audio_loop::AudioLoopEvent::StartPositionImmediate(7.38, audio_handles.audio_handle.clone()));
//!
//!     // Set the new end position which gets activated immediately.  This is stop stop the current loop if the new stop
//!     // position is before the current start position.
//!     audio_events.send(audio_loop::AudioLoopEvent::EndPositionImmediate(7.38, audio_handles.audio_handle.clone()));
//! }

use std::sync::{Arc, RwLock};

use bevy::asset::AssetLoader;
use bevy::audio::{AddAudioSource, AudioLoader, Source};
use bevy::prelude::*;

#[derive(Asset, TypePath)]
pub struct LoopableAudioSource {
    inner: AudioSource,
    extracted_data: Vec<<bevy::prelude::AudioSource as Decodable>::DecoderItem>,
    loop_start: Arc<RwLock<f32>>,
    loop_end: Arc<RwLock<f32>>,
    future_loop_start: Arc<RwLock<Option<f32>>>,
    future_loop_end: Arc<RwLock<Option<f32>>>,
    sample_rate: u32,
    channels: u16,
    current_position: Arc<RwLock<usize>>,
}

impl LoopableAudioSource {
    pub fn new(audio_source: AudioSource, loop_start: f32, loop_end: f32) -> Self {
        let sample_rate = audio_source.decoder().sample_rate();
        let channels = audio_source.decoder().channels();
        let extracted_data = audio_source.decoder().collect::<Vec<_>>();
        Self {
            inner: audio_source,
            extracted_data,
            loop_start: Arc::new(RwLock::new(loop_start)),
            loop_end: Arc::new(RwLock::new(loop_end)),
            future_loop_start: Arc::new(RwLock::new(None)),
            future_loop_end: Arc::new(RwLock::new(None)),
            sample_rate,
            channels,
            current_position: Arc::new(RwLock::new(0)),
        }
    }

    pub fn set_loop_start_immediate(&mut self, loop_start: f32) {
        *self.loop_start.write().unwrap() = loop_start;
    }

    pub fn set_loop_end_immediate(&mut self, loop_end: f32) {
        *self.loop_end.write().unwrap() = loop_end;
    }

    pub fn set_loop_start(&mut self, loop_start: f32) {
        *self.future_loop_start.write().unwrap() = Some(loop_start);
    }

    pub fn set_loop_end(&mut self, loop_end: f32) {
        *self.future_loop_end.write().unwrap() = Some(loop_end);
    }

    pub fn add_loop_offset(&mut self, offset: f32) {
        let loop_start =
            (*self.future_loop_start.read().unwrap()).unwrap_or(*self.loop_start.read().unwrap());
        let loop_end =
            (*self.future_loop_end.read().unwrap()).unwrap_or(*self.loop_end.read().unwrap());
        let range = loop_end - loop_start;
        let mut new_loop_start = loop_start + offset;
        let mut new_loop_end = loop_end + offset;
        if loop_start < 0.0 {
            new_loop_start = 0.0;
            new_loop_end = range;
        }
        self.set_loop_start(new_loop_start);
        self.set_loop_end(new_loop_end);
    }

    pub fn set_position(&mut self, position: f32) {
        *self.current_position.write().unwrap() =
            (position * self.sample_rate() as f32 * self.channels() as f32) as usize;
    }

    pub fn get_position(&self) -> f32 {
        *self.current_position.read().unwrap() as f32
            / self.sample_rate() as f32
            / self.channels() as f32
    }

    pub fn move_position(&mut self, offset: f32) {
        self.set_position((self.get_position() + offset).max(0.0));
    }

    pub fn set_loop_and_pos_immediate(&mut self, loop_start: f32, loop_end: f32, position: f32) {
        self.set_loop_start_immediate(loop_start);
        self.set_loop_end_immediate(loop_end);
        self.set_position(position);
        *self.future_loop_start.write().unwrap() = None;
        *self.future_loop_end.write().unwrap() = None;
    }

    pub fn move_loop_offset(&mut self, offset: f32) {
        let loop_start =
            (*self.future_loop_start.read().unwrap()).unwrap_or(*self.loop_start.read().unwrap());
        let loop_end =
            (*self.future_loop_end.read().unwrap()).unwrap_or(*self.loop_end.read().unwrap());
        let range = loop_end - loop_start;
        let position_offset = self.get_position() - loop_start;
        let mut new_loop_start = loop_start + offset;
        let mut new_loop_end = loop_end + offset;
        if loop_start < 0.0 {
            new_loop_start = 0.0;
            new_loop_end = range;
        }
        self.set_loop_start_immediate(new_loop_start);
        self.set_loop_end_immediate(new_loop_end);
        self.set_position(new_loop_start + position_offset);
    }
}

impl Iterator for LoopableAudioSource {
    type Item = <bevy::prelude::AudioSource as Decodable>::DecoderItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut loop_start = *self.loop_start.read().unwrap();
        let loop_end = *self.loop_end.read().unwrap();
        if *self.current_position.read().unwrap() >= self.extracted_data.len() {
            *self.current_position.write().unwrap() =
                (loop_start * self.sample_rate() as f32 * self.channels() as f32) as usize;
        }
        let seconds = *self.current_position.read().unwrap() as f32
            / self.sample_rate() as f32
            / self.channels() as f32;
        if seconds > loop_end {
            let mut future_loop_start = self.future_loop_start.write().unwrap();
            let mut future_loop_end = self.future_loop_end.write().unwrap();

            if let Some(future_loop_start) = *future_loop_start {
                *self.loop_start.write().unwrap() = future_loop_start;
                loop_start = future_loop_start;
            }
            if let Some(future_loop_end) = *future_loop_end {
                *self.loop_end.write().unwrap() = future_loop_end;
            }
            *future_loop_start = None;
            *future_loop_end = None;

            *self.current_position.write().unwrap() =
                (loop_start * self.sample_rate() as f32 * self.channels() as f32) as usize;
        }
        let result = Some(self.extracted_data[*self.current_position.read().unwrap()]);
        *self.current_position.write().unwrap() += 1;
        result
    }
}

impl Source for LoopableAudioSource {
    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }
}

impl Decodable for LoopableAudioSource {
    type DecoderItem = <LoopableAudioSource as Iterator>::Item;
    type Decoder = LoopableAudioSource;

    fn decoder(&self) -> Self::Decoder {
        LoopableAudioSource {
            inner: self.inner.clone(),
            extracted_data: self.extracted_data.clone(),
            loop_start: self.loop_start.clone(),
            loop_end: self.loop_end.clone(),
            future_loop_start: self.future_loop_start.clone(),
            future_loop_end: self.future_loop_end.clone(),
            sample_rate: self.sample_rate,
            channels: self.channels,
            current_position: self.current_position.clone(),
        }
    }
}

#[derive(Default)]
pub struct LoopedAudioLoader;
impl AssetLoader for LoopedAudioLoader {
    type Asset = LoopableAudioSource;

    type Settings = ();

    type Error = bevy::tasks::futures_lite::io::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let audio_source = AudioLoader.load(reader, settings, load_context).await?;
            Ok(LoopableAudioSource::new(audio_source, 0.0, f32::MAX))
        })
    }
}

pub struct AudioLoopPlugin;
impl Plugin for AudioLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<LoopableAudioSource>()
            .init_asset_loader::<LoopedAudioLoader>()
            .add_event::<AudioLoopEvent>()
            .add_systems(PostUpdate, audio_loop_event_handler);
    }
}

#[derive(Event, Clone)]
pub enum AudioLoopEvent {
    StartPositionImmediate(f32, Handle<LoopableAudioSource>),
    EndPositionImmediate(f32, Handle<LoopableAudioSource>),
    StartPosition(f32, Handle<LoopableAudioSource>),
    EndPosition(f32, Handle<LoopableAudioSource>),
    LoopOffset(f32, Handle<LoopableAudioSource>),
    LoopOffsetImmediate(f32, Handle<LoopableAudioSource>),
    LoopPosition(f32, f32, f32, Handle<LoopableAudioSource>),
}

pub fn audio_loop_event_handler(
    mut audio_loops: ResMut<Assets<LoopableAudioSource>>,
    mut audio_loop_events: EventReader<AudioLoopEvent>,
    mut buffered_events: Local<Vec<AudioLoopEvent>>,
) {
    let mut rebuffered_events = Vec::new();
    for event in buffered_events.drain(..) {
        if !process_event(&event, audio_loops.as_mut()) {
            rebuffered_events.push(event.clone());
        }
    }
    buffered_events.append(&mut rebuffered_events);
    for event in audio_loop_events.read() {
        if !process_event(event, audio_loops.as_mut()) {
            buffered_events.push(event.clone());
        }
    }
}

fn process_event(event: &AudioLoopEvent, audio_loops: &mut Assets<LoopableAudioSource>) -> bool {
    match event {
        AudioLoopEvent::StartPositionImmediate(position, handle) => {
            if let Some(audio_loop) = audio_loops.get_mut(handle.clone()) {
                audio_loop.set_loop_start_immediate(*position);
            } else {
                return false;
            }
        }
        AudioLoopEvent::EndPositionImmediate(position, handle) => {
            if let Some(audio_loop) = audio_loops.get_mut(handle.clone()) {
                audio_loop.set_loop_end_immediate(*position);
            } else {
                return false;
            }
        }
        AudioLoopEvent::StartPosition(position, handle) => {
            if let Some(audio_loop) = audio_loops.get_mut(handle.clone()) {
                audio_loop.set_loop_start(*position);
            } else {
                return false;
            }
        }
        AudioLoopEvent::EndPosition(position, handle) => {
            if let Some(audio_loop) = audio_loops.get_mut(handle.clone()) {
                audio_loop.set_loop_end(*position);
            } else {
                return false;
            }
        }
        AudioLoopEvent::LoopOffset(offset, handle) => {
            if let Some(audio_loop) = audio_loops.get_mut(handle.clone()) {
                audio_loop.add_loop_offset(*offset);
            } else {
                return false;
            }
        }
        AudioLoopEvent::LoopOffsetImmediate(offset, handle) => {
            if let Some(audio_loop) = audio_loops.get_mut(handle.clone()) {
                audio_loop.move_loop_offset(*offset);
            } else {
                return false;
            }
        }
        AudioLoopEvent::LoopPosition(loop_start, loop_end, position, handle) => {
            if let Some(audio_loop) = audio_loops.get_mut(handle.clone()) {
                audio_loop.set_loop_and_pos_immediate(*loop_start, *loop_end, *position);
            } else {
                return false;
            }
        }
    }
    true
}
