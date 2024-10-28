use bevy::prelude::*;

use calc_sim::cam_ui::setup;
use calc_sim::cam_world::{pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::PanOrbitState;
use calc_sim::game_env::spawn_gltf;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_gltf)
        .add_systems(Startup, spawn_3d_camera)
        .add_systems(Update,
            pan_orbit_camera
                .run_if(any_with_component::<PanOrbitState>))
        .run();
}