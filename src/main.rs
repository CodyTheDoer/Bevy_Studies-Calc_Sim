use bevy::prelude::*;
use bevy::input::common_conditions::*;

use calc_sim::cam_ui::setup_ui;
use calc_sim::cam_ui::CameraUi;
use calc_sim::cam_world::{pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::CameraWorld;
use calc_sim::cam_world::PanOrbitState;
use calc_sim::game_env::spawn_gltf;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
        ))
        .add_systems(
            Startup, 
            (
                setup_ui,
                spawn_gltf,
                spawn_3d_camera,
            )
        )
        .add_systems(
            Update, 
            (
                pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
                // raycast_system.run_if(ButtonInput(MouseButton::Left)),
            )
        )
        .run();
}