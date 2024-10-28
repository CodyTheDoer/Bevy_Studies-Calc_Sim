use bevy::prelude::*;
use bevy::input::common_conditions::*;

use calc_sim::cam_ui::setup_ui;
use calc_sim::cam_ui::CameraUi;
use calc_sim::cam_world::{pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::CameraWorld;
// use calc_sim::cam_world::PanOrbitState;
// use calc_sim::game_env::spawn_gltf;

fn main() {
    // App::new()
    //     .add_plugins((
    //         DefaultPlugins,
    //     ))
    //     .add_systems(
    //         Startup, 
    //         (
    //             setup,
    //             setup_ui,
    //             spawn_gltf,
    //             // spawn_3d_camera,
    //         )
    //     )
    //     .add_systems(
    //         Update, 
    //         (
    //             // pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
    //             draw_cursor,
    //         )
    //     )
    //     .run();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup,
            setup_ui,
        ))
        .add_systems(Update, draw_cursor)
        .run();
}

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraWorld,
    ));
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);
}