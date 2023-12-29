use bevy::prelude::*;

/// Allows to automatically handle asset loading.
///
/// This should be implemented for structs. The struct can only contain assets of the same type
/// and they should be set as Handle.  For example Handle<Image>.
pub trait EasyAssetLoader {
    type AssetType: Asset;

    fn asset_mapper() -> &'static [(&'static str, &'static str)];
}

/// Keeps track of the overall asset loading.
#[derive(Resource)]
pub struct LoadAssets<S: States> {
    /// Total bullshit at the moment.
    pub splash_image_path: String,

    /// Also total bullshit.
    pub splash_image: Handle<Image>,

    /// This state will be set when all assets are loaded.
    pub target_state: S,

    /// Currently loaded assets.
    pub asset_count: u32,

    /// Overall amount of assets.
    pub current_loaded_assets: u32,
}
impl<S: States> LoadAssets<S> {
    pub fn new(path: String, target_state: S) -> Self {
        Self {
            splash_image_path: path,
            splash_image: Handle::default(),
            target_state,
            asset_count: 0,
            current_loaded_assets: 0,
        }
    }
}

/// Starts to load assets for a struct which implements EasyAssetLoader.
pub fn load_assets_init<A: EasyAssetLoader + Struct + Resource>(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<A>,
) {
    for (asset, path) in A::asset_mapper() {
        let handle: Handle<A::AssetType> = asset_server.load(*path);
        if let Some(attribute) = assets.field_mut(asset) {
            attribute.apply(&handle);
            bevy::log::info!("Start loading field {} for asset loading", asset);
        } else {
            bevy::log::error!("Could initialize loading field {} for asset loading", asset);
        }
    }
}

/// Resets the loading status on each frame.
///
/// This allows the systems which check the current state to increment these values again.
pub fn load_assets_reset<S: States>(mut load_assets: ResMut<LoadAssets<S>>) {
    load_assets.current_loaded_assets = 0;
    load_assets.asset_count = 0;
    bevy::log::info!("Reset loading status");
}

/// Update the load_assets stats for a specific EasyAssetLoader.
///
/// It will only increment the values and expects that the values were set to 0 before
/// on each frame.
pub fn load_assets_check<A: EasyAssetLoader + Struct + Resource, S: States>(
    asset_server: Res<AssetServer>,
    assets: ResMut<A>,
    mut load_assets: ResMut<LoadAssets<S>>,
) {
    for (asset_attribute, _) in A::asset_mapper() {
        bevy::log::info!("Check asset loading status for {}", asset_attribute);
        if let Some(asset) = assets.field(asset_attribute) {
            let any_asset = asset.as_any();
            if let Some(handle) = any_asset.downcast_ref::<Handle<A::AssetType>>() {
                if asset_server.is_loaded_with_dependencies(handle) {
                    load_assets.current_loaded_assets += 1;
                    bevy::log::info!("Asset {} is loaded", asset_attribute);
                } else {
                    bevy::log::info!("Asset {} not loaded yet", asset_attribute);
                }
                load_assets.asset_count += 1;
            } else {
                bevy::log::error!(
                    "Could not downcast field {} for asset loading",
                    asset_attribute
                );
            }
        } else {
            bevy::log::error!("Could not load field {} for asset loading", asset_attribute);
        }
    }
}

/// Checks if all assets were loaded and sets the destination state.
pub fn load_assets_final_check<S: States + Clone>(
    mut load_assets: ResMut<LoadAssets<S>>,
    mut state: ResMut<NextState<S>>,
) {
    if load_assets.current_loaded_assets == load_assets.asset_count {
        state.set(load_assets.target_state.clone());
        bevy::log::info!(
            "All assets were loaded successfully ({})",
            load_assets.asset_count
        );
        load_assets.current_loaded_assets = 0;
        load_assets.asset_count = 0;
    } else {
        bevy::log::info!(
            "Not all assets were loaded ({}/{})",
            load_assets.current_loaded_assets,
            load_assets.asset_count
        );
    }
}

/// General plugin to to check the loading state.
///
/// It needs the loading game state and the destination game state.  During the loading state it will
/// prepare the loading system and check if all assets were loaded.  If they were laoded, it will
/// set the destination state.
///
/// Should be combined with LoadingPluginAssets.
pub struct LoadingPlugin<S: States + Clone>(pub S, pub S);
impl<S: States> Plugin for LoadingPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadAssets::new("".into(), self.1.clone()))
            .add_systems(
                Update,
                (load_assets_reset::<S>, load_assets_final_check::<S>)
                    .chain()
                    .run_if(in_state(self.0.clone())),
            );
    }
}

/// Sets up asset loading for a EasyAssetLoader struct.
///
/// It will start to load the assets of the EasyAssetLoader struct and check the loading status on each frame.
///
/// Must be combined with LoadingPlugin.
pub struct LoadPluginAssets<A: EasyAssetLoader + Struct + Resource + Clone, S: States + Clone>(
    pub A,
    pub S,
);
impl<A: EasyAssetLoader + Struct + Resource + Clone, S: States + Clone> Plugin
    for LoadPluginAssets<A, S>
{
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone())
            .add_systems(OnEnter(self.1.clone()), load_assets_init::<A>)
            .add_systems(
                Update,
                load_assets_check::<A, S>
                    .run_if(in_state(self.1.clone()))
                    .after(load_assets_reset::<S>)
                    .before(load_assets_final_check::<S>),
            );
    }
}
