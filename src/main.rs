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
        .add_state(CalculatorState::Init) // Start with the Init state
        .add_system_set(SystemSet::on_update(CalculatorState::Add).with_system(add_system))
        .add_system_set(SystemSet::on_update(CalculatorState::Subtract).with_system(subtract_system))
        .add_system_set(SystemSet::on_update(CalculatorState::Clear).with_system(clear_system))
        .add_system_set(SystemSet::on_update(CalculatorState::Init).with_system(init_system));
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum CalculatorState {
    Init,
    Add,
    Subtract,
    Multiply,
    Divide,
    Clear,
}

fn add_system(
    mut var: ResMut<SumVariable>,
    mut sum: ResMut<SumCurrent>,
    mut op: ResMut<OpIndex>,
) {
    info!("Running Add system");
    // Add your logic here to update the sum
}

fn subtract_system(
    mut var: ResMut<SumVariable>,
    mut sum: ResMut<SumCurrent>,
    mut op: ResMut<OpIndex>,
) {
    info!("Running Init system");
    // Handle Init logic here
}

fn clear_system(
    mut var: ResMut<SumVariable>,
    mut sum: ResMut<SumCurrent>,
    mut op: ResMut<OpIndex>,
) {
    info!("Running Add system");
    info!("match_str: Clear");
    SumCurrent::new();
    while var.var.len() > 0 {
        var.var.pop();
    }
    var.review(); // Reviews the Vec of numbers stored in the Variable Vec and the period index.
}

fn init_system(
    mut var: ResMut<SumVariable>,
    mut sum: ResMut<SumCurrent>,
    mut op: ResMut<OpIndex>,
) {
    info!("Running Init system");
    // Handle Init logic here
}