use bevy::{prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::*,
    window::{PresentMode, WindowTheme},
    winit::WinitWindows,
};

use winit::window::Icon;

use calc_sim::{OpIndex, SumCurrent, SumVariable};

use calc_sim::calculator::{cycle_screen_albedo, screen_albedo};
use calc_sim::calculator::{CurrentMeshColor, ScreenAlbedoState};

use calc_sim::cam_ui::setup_ui;

use calc_sim::cam_world::{draw_cursor, pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::PanOrbitState;

use calc_sim::cam_calc_screen::{setup_calc_interface_projection, update_sum_text, update_var_text};

use calc_sim::game_env::{button_animation_system, dim_while_clicked, fire_ray, handle_asset_events, release_ray, body_animation_system, spawn_gltf};
use calc_sim::game_env::CountdownCycle;

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Calculator Simulator".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1280., 720.).into(),
                    resizable: true,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: true,
                        ..Default::default()
                    },
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .init_resource::<CurrentMeshColor>()
        .init_resource::<CountdownCycle>()
        .init_resource::<ScreenAlbedoState>()
        .insert_resource(SumCurrent::new())
        .insert_resource(SumVariable::new())
        .insert_resource(OpIndex::new())
        .add_systems(Startup, set_window_icon)
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
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)));
        app.run();
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon/calc_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}