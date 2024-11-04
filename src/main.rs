use bevy::{prelude::*,
    asset::{AssetEvent, Assets, Handle},
    input::common_conditions::*,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
};

use bevy_mod_raycast::prelude::*;

use calc_sim::{FlexInput, OpIndex, SumCurrent, SumVariable};

use calc_sim::calculator::{cycle_screen_albedo, screen_albedo};
use calc_sim::calculator::{CalcButtons, CurrentMeshColor, MeshColor, ScreenAlbedoState};

use calc_sim::cam_ui::setup_ui;
use calc_sim::cam_ui::CameraUi;

use calc_sim::cam_world::{draw_cursor, pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::{CameraWorld, PanOrbitState};

use calc_sim::cam_calc_screen::{setup_calc_interface_projection, update_sum_text, update_var_text};

use calc_sim::game_env::{button_animation_system, fire_ray, handle_asset_events, release_ray, body_animation_system, spawn_gltf};
use calc_sim::game_env::{CountdownCycle, Interactable, Loaded};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CurrentMeshColor>()
        .init_resource::<CountdownCycle>()
        .init_resource::<ScreenAlbedoState>()
        .insert_resource(SumCurrent::new())
        .insert_resource(SumVariable::new())
        .insert_resource(OpIndex::new())
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, spawn_gltf)
        .add_systems(Startup, spawn_3d_camera)
        .add_systems(Startup, setup_calc_interface_projection)
        .add_systems(Update, button_animation_system)
        .add_systems(Update, body_animation_system)
        .add_systems(Update, draw_cursor)
        .add_systems(Update, update_sum_text)
        .add_systems(Update, update_var_text)
        .add_systems(Update, handle_asset_events)
        .add_systems(Update, screen_albedo)
        .add_systems(Update, dim_while_clicked.run_if(|state: Res<ScreenAlbedoState>| state.should_run_dim()))
        .add_systems(Update, cycle_screen_albedo.run_if(|state: Res<ScreenAlbedoState>| state.should_run_cycle()))
        .add_systems(Update, pan_orbit_camera.run_if(any_with_component::<PanOrbitState>))
        .add_systems(Update, release_ray.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)))
        .run();
}

fn dim_while_clicked(
    screen: Res<ScreenAlbedoState>,
    mut op_index: ResMut<OpIndex>,
) {
    op_index.screen_color = 5;
    // screen_albedo_state.state = 0;
}