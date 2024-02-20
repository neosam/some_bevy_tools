use bevy::prelude::*;
use bevy_helper_tools::loading as easy_loading;

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

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(easy_loading::LoadingPlugin(
            GameState::Loading,
            GameState::InGame,
        ))
        .add_plugins(easy_loading::LoadPluginAssets(
            TextureAssets::default(),
            GameState::Loading,
        ))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::InGame), init_ingame)
        .run();
}

pub fn init_ingame(mut commands: Commands, assets: Res<TextureAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: assets.ducky.clone(),
        transform: Transform::from_xyz(300.0, 0.0, 0.0),
        ..Default::default()
    });
}
