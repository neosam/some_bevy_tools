//! Module which contains tools which help to despawn entities.
//!
//! ## Example
//! ```rust
//! use bevy::prelude::*;
//! use some_bevy_tools::despawn;
//!
//! fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     commands.spawn(Camera2dBundle::default());
//!     commands.spawn((
//!         SpriteBundle {
//!             texture: asset_server.load("ducky.png"),
//!             ..Default::default()
//!         },
//!         despawn::AutoDespawn::new(3.0),
//!     ));
//! }
//!
//! App::new()
//!     //.add_plugins(DefaultPlugins)
//!     .add_plugins(despawn::AutoDespawnPlugin)
//!     .add_systems(Startup, setup);
//!     //.run();
//!
//! ```

use bevy::prelude::*;

/// Automatically despawns an entity after a certain amount of time.
#[derive(Debug, Component)]
pub enum AutoDespawn {
    Timer(Timer),
    Frames(u32),
}
impl AutoDespawn {
    /// Create a new auto despawn which despwns in the given duration which is in seconds.
    ///
    /// # Deprecated
    /// Use with_duration instead because it is also possible to auto despawn in a few frames.
    #[deprecated]
    pub fn new(duration: f32) -> Self {
        Self::with_duration(duration)
    }

    /// Create a new auto despawn which despwns in the given duration which is in seconds.
    pub fn with_duration(duration: f32) -> Self {
        AutoDespawn::Timer(Timer::from_seconds(duration, TimerMode::Once))
    }

    pub fn with_frames(frames: u32) -> Self {
        AutoDespawn::Frames(frames)
    }
}

/// System that automatically despawns entities which contains the `AutoDespawn` component.
pub fn auto_despawn_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AutoDespawn)>,
    time: Res<Time>,
) {
    for (entity, mut auto_despawn) in query.iter_mut() {
        match auto_despawn.as_mut() {
            AutoDespawn::Timer(timer) => {
                if timer.tick(time.delta()).just_finished() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            AutoDespawn::Frames(frames) => {
                if *frames == 0 {
                    commands.entity(entity).despawn_recursive();
                } else {
                    *frames -= 1;
                }
            }
        }
    }
}

/// Plugin that automatically despawns entities which contains the `AutoDespawn` component.
pub struct AutoDespawnPlugin;
impl Plugin for AutoDespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, auto_despawn_system);
    }
}

/// Automatically cleans up entities when the defined state is exited.
#[derive(Debug, Component)]
pub struct Cleanup<S>(pub S);

/// System that automatically cleans up entities when the defined state is exited.
pub fn cleanup_system<S: States + Eq>(state: S) -> impl Fn(Commands, Query<(Entity, &Cleanup<S>)>) {
    move |mut commands: Commands, query: Query<(Entity, &Cleanup<S>)>| {
        for (entity, cleanup) in query.iter() {
            if cleanup.0 == state {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

/// Plugin that automatically cleans up entities when the defined state is exited.
pub struct CleanupPlugin<S>(pub S);
impl<S: Clone + States + Send + Sync + 'static> Plugin for CleanupPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(self.0.clone()), cleanup_system(self.0.clone()));
    }
}
