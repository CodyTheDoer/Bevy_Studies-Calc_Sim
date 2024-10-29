use bevy::prelude::*;
use bevy::input::common_conditions::*;

use bevy_mod_raycast::prelude::*;

// use calc_sim::{add, subtract, multiply, divide};
use calc_sim::{FlexInput, OpIndex, SumCurrent, SumVariable};

use calc_sim::cam_ui::setup_ui;
use calc_sim::cam_ui::CameraUi;

use calc_sim::cam_world::{draw_cursor, pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::{CameraWorld, PanOrbitState};

use calc_sim::game_env::{fire_ray, spawn_gltf};
use calc_sim::game_env::{CalcButtons, Interactable};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SumCurrent::new())
        .insert_resource(SumVariable::new())
        .insert_resource(OpIndex::new())
        .add_systems(
            Startup, 
            (
                setup_backend,
                setup_ui,
                spawn_gltf,
                // add_interactable_to_meshes.after(spawn_gltf),
                spawn_3d_camera,
            )
        )
        .add_systems(
            Update, 
            (
                draw_cursor,
                pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
                fire_ray.run_if(input_just_released(MouseButton::Left)),
            )
        )
        .run();
}

fn setup_backend(
    mut op: ResMut<OpIndex>,
) {
    op.index = 0;
}