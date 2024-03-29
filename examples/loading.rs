use bevy::prelude::*;
use some_bevy_tools::{
    audio_loop::{AudioLoopPlugin, LoopableAudioSource},
    loading as easy_loading,
};

#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
}

#[derive(Resource, Default, Reflect, Clone)]
pub struct TextureAssets {
    pub ducky: Handle<Image>,
}
impl easy_loading::EasyAssetLoader for TextureAssets {
    type AssetType = Image;
    fn asset_mapper() -> &'static [(&'static str, &'static str)] {
        &[("ducky", "ducky.png")]
    }
}

#[derive(Resource, Default, Reflect, Clone)]
pub struct AudioAssets {
    pub music: Handle<LoopableAudioSource>,
}
impl easy_loading::EasyAssetLoader for AudioAssets {
    type AssetType = LoopableAudioSource;
    fn asset_mapper() -> &'static [(&'static str, &'static str)] {
        &[("music", "ehh-ehh.ogg")]
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioLoopPlugin)
        .add_plugins(easy_loading::LoadingPlugin(
            GameState::Loading,
            GameState::InGame,
        ))
        .add_plugins(easy_loading::LoadPluginAssets(
            TextureAssets::default(),
            GameState::Loading,
        ))
        .add_plugins(easy_loading::LoadPluginAssets(
            AudioAssets::default(),
            GameState::Loading,
        ))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::InGame), init_ingame)
        .run();
}

pub fn init_ingame(mut commands: Commands, assets: Res<TextureAssets>, audio: Res<AudioAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: assets.ducky.clone(),
        transform: Transform::from_xyz(300.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn(AudioSourceBundle {
        source: audio.music.clone(),
        ..Default::default()
    });
}
