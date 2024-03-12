//! Demonstrates how to set up a SBS 3D camera in bevy.
//!
//! SBS allows XR glasses to display 3D content by showing the left half of the screen
//! on the left eye and the right half on the right eye.
//!
//! The example shows how to set up a SBS camera in bevy with the bevy_helper_tools.
//! SBS mode and single camera mode can be toggled with the space bar.

use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::window::WindowResized;
use some_bevy_tools::sbs_3d;
use some_bevy_tools::split_screen;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(sbs_3d::Sbs3DPlugin)
        .add_systems(Startup, (setup_sbs, setup_object, register_systems))
        .add_systems(Update, (rotate, switch_state))
        .run();
}

/// Entities which should rotate.
#[derive(Component)]
struct Rotate;

/// Marker for all SBS entities required for cleanup.
#[derive(Component)]
struct SbsCamera;

/// Marker for all single camera entities required for cleanup.
#[derive(Component)]
struct SingleCamera;

/// State of the SBS mode to toggle between Single and SBS state.
#[derive(Resource, Default)]
enum SbsState {
    #[default]
    Enabled,
    Disabled,
}

/// Hold system IDs required to switch between SBS and single camera mode.
///
/// Toggling between SBS and single camera is done by calling one-shot systems.
/// This resource stores the SystemId's which allows other system to call them.
#[derive(Resource)]
struct SystemIds {
    setup_sbs: SystemId,
    cleanup_sbs: SystemId,
    setup_single: SystemId,
    cleanup_single: SystemId,
}

/// Register the one-shot systems and provide them as a SystemIds resource.
fn register_systems(world: &mut World) {
    let setup_sbs = world.register_system(setup_sbs);
    let cleanup_sbs = world.register_system(cleanup_sbs);
    let setup_single = world.register_system(setup_single);
    let cleanup_single = world.register_system(cleanup_single);
    world.insert_resource(SystemIds {
        setup_sbs,
        cleanup_sbs,
        setup_single,
        cleanup_single,
    });
}

/// Setup for the SBS camera.
///
/// It requires a `LeftCamera` and a `RigthCamera` as actual cameras used in Bevy
/// and a `SbsCamera` as the camera used to set the transform for the cameras.
fn setup_sbs(
    mut commands: Commands,
    window_query: Query<(Entity, &Window)>,
    mut window_resized: EventWriter<WindowResized>,
) {
    commands.spawn((
        Camera3dBundle::default(),
        split_screen::LeftCamera,
        SbsCamera,
    ));
    commands.spawn((
        Camera3dBundle::default(),
        split_screen::RightCamera,
        SbsCamera,
    ));
    commands.spawn((
        sbs_3d::SbsCameraBundle::from_transform_and_gap(
            Transform::from_xyz(1.0, 1.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            0.3,
        ),
        SbsCamera,
    ));
    if let Ok((entity, window)) = window_query.get_single() {
        window_resized.send(WindowResized {
            window: entity,
            width: window.width(),
            height: window.height(),
        });
    }
}

/// Despawn all SBS cameras.
fn cleanup_sbs(mut commands: Commands, sbs_cameras: Query<Entity, With<SbsCamera>>) {
    for entity in sbs_cameras.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Setup for a simple single camera as usual in bevy.
fn setup_single(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 1.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        SingleCamera,
    ));
}

/// Despawn the single camera.
fn cleanup_single(mut commands: Commands, single_cameras: Query<Entity, With<SingleCamera>>) {
    for entity in single_cameras.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Spawn the cube and the light.
fn setup_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial { ..default() });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: debug_material,
            ..Default::default()
        },
        Rotate,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

/// Rotate entities which have a `Rotate` component as marker.
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotate>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_seconds());
    }
}

/// Toggles between single and SBS camera mode when the space bar is pressed.
///
/// For this, it will use one-shot systems to despawn and spawn the cameras.
fn switch_state(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut state: Local<SbsState>,
    system_ids: Res<SystemIds>,
) {
    if input.just_pressed(KeyCode::Space) {
        match *state {
            SbsState::Enabled => {
                commands.run_system(system_ids.cleanup_sbs);
                commands.run_system(system_ids.setup_single);
                *state = SbsState::Disabled;
            }
            SbsState::Disabled => {
                commands.run_system(system_ids.cleanup_single);
                commands.run_system(system_ids.setup_sbs);
                *state = SbsState::Enabled
            }
        }
    }
}
