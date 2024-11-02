use bevy::prelude::*;
use bevy::input::common_conditions::*;

use bevy_mod_raycast::prelude::*;

use calc_sim::{FlexInput, OpIndex, SumCurrent, SumVariable};

use calc_sim::cam_ui::{setup_ui, update_sum_text, update_var_text};
use calc_sim::cam_ui::CameraUi;

use calc_sim::cam_world::{draw_cursor, pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::{CameraWorld, PanOrbitState};

use calc_sim::game_env::{button_animation_system, fire_ray, handle_asset_events, screen_albedo, spawn_gltf, update_screen_albedo};
use calc_sim::game_env::{CalcButtons, Countdown, CurrentMeshColor, Interactable, Loaded, ScreenAlbedoState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CurrentMeshColor>()
        .init_resource::<Countdown>()
        .init_resource::<ScreenAlbedoState>()
        .insert_resource(SumCurrent::new())
        .insert_resource(SumVariable::new())
        .insert_resource(OpIndex::new())
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
                button_animation_system,
                draw_cursor,
                update_sum_text,
                update_var_text,
                handle_asset_events,
                screen_albedo, 
                update_screen_albedo.run_if(|state: Res<ScreenAlbedoState>| state.should_run()),
                pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
                fire_ray.run_if(input_just_released(MouseButton::Left)),
            )
        )
        .run();
}
















