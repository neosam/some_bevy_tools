use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use some_bevy_tools::{third_party_camera, third_party_controller};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(third_party_controller::ThirdPartyControllerPlugin)
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 1.0)))
        .add_systems(Startup, (setup_object, grab_cursor))
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                third_party_controller::move_controller_plane,
            ),
        )
        .run();
}

fn grab_cursor(mut query_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = query_windows.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

/// Spawn the cube and the light.
fn setup_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grey_material = materials.add(StandardMaterial { ..default() });
    let green_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.0),
        ..default()
    });
    let red_materiral = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        ..default()
    });
    let target = commands
        .spawn((PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: grey_material,
            ..Default::default()
        },))
        .id();

    commands.spawn((PbrBundle {
        mesh: meshes.add(Sphere::default()),
        material: red_materiral.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -20.0),
        ..Default::default()
    },));
    commands.spawn((PbrBundle {
        mesh: meshes.add(Sphere::default()),
        material: red_materiral.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 20.0),
        ..Default::default()
    },));
    commands.spawn((PbrBundle {
        mesh: meshes.add(Sphere::default()),
        material: red_materiral.clone(),
        transform: Transform::from_xyz(-20.0, 0.0, 0.0),
        ..Default::default()
    },));
    commands.spawn((PbrBundle {
        mesh: meshes.add(Sphere::default()),
        material: red_materiral,
        transform: Transform::from_xyz(20.0, 0.0, 0.0),
        ..Default::default()
    },));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::default()),
        material: green_material,
        transform: Transform::from_xyz(0.0, -1.0, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
        ..Default::default()
    },));

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

    commands.spawn((
        Camera3dBundle::default(),
        third_party_camera::ThirdPartyCamera {
            target,
            distance: 10.0,
            rotate_y: std::f32::consts::PI / 4.0,
            rotate_x: std::f32::consts::PI / 8.0,
        },
        third_party_controller::ThirdPartyController {
            min_distance: 1.0,
            max_distance: 40.0,
        },
    ));
}
