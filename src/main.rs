use bevy::prelude::*;
use bevy::input::common_conditions::*;

use bevy_mod_raycast::prelude::*;

use calc_sim::{add, subtract, multiply, divide};
use calc_sim::FlexInput;

use calc_sim::cam_ui::setup_ui;
use calc_sim::cam_ui::CameraUi;

use calc_sim::cam_world::{pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::{CameraWorld, PanOrbitState};

use calc_sim::game_env::spawn_gltf;
use calc_sim::game_env::Interactable;

#[derive(Debug)]
enum CalcButtons {
    ButtonSum,
    ButtonAdd,
    ButtonSubtract,
    ButtonMultiply,
    ButtonDivide,
    ButtonClear,
    ButtonDecimal,
    ButtonNum0,
    ButtonNum1,
    ButtonNum2,
    ButtonNum3,
    ButtonNum4,
    ButtonNum5,
    ButtonNum6,
    ButtonNum7,
    ButtonNum8,
    ButtonNum9,
    NoneButton_Body,
    NoneButton_Screen,
    NoneButton_LightPanel,
}

impl CalcButtons {
    fn from_index(index: u32) -> Option<CalcButtons> {
        match index {
            42 => Some(CalcButtons::ButtonSum),
            43 => Some(CalcButtons::ButtonAdd),
            44 => Some(CalcButtons::ButtonSubtract),
            45 => Some(CalcButtons::ButtonMultiply),
            46 => Some(CalcButtons::ButtonDivide),
            48 => Some(CalcButtons::ButtonClear),
            50 => Some(CalcButtons::ButtonDecimal),
            49 => Some(CalcButtons::ButtonNum0),
            52 => Some(CalcButtons::ButtonNum1),
            53 => Some(CalcButtons::ButtonNum2),
            54 => Some(CalcButtons::ButtonNum3),
            56 => Some(CalcButtons::ButtonNum4),
            57 => Some(CalcButtons::ButtonNum5),
            58 => Some(CalcButtons::ButtonNum6),
            60 => Some(CalcButtons::ButtonNum7),
            61 => Some(CalcButtons::ButtonNum8),
            62 => Some(CalcButtons::ButtonNum9),
            39 => Some(CalcButtons::NoneButton_Body),
            64 => Some(CalcButtons::NoneButton_Screen),
            65 => Some(CalcButtons::NoneButton_LightPanel),
            _ => None, // Handle invalid index
        }
    }

    fn button_info(&self) {
        info!("Button Clicked: {:?}", self);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup, 
            (
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

fn draw_cursor(
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {    
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => {
            warn!("No CameraWorld found or multiple CameraWorlds detected.");
            return;
        },
    };

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let hits = raycast.cast_ray(ray, &RaycastSettings::default());

    if let Some((_, intersection)) = hits.first() {
        // Get the intersection point.
        let point = intersection.position();

        // Draw a circle at the intersection point using Gizmos (just above the surface).
        let up = Dir3::Y; 
        gizmos.circle(point + up * 0.05, up, 0.2, Color::WHITE);
    }
}

use bevy::ecs::event::EventReader;

fn fire_ray(
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    windows: Query<&Window>,
    interactable_query: Query<Entity, With<Interactable>>,
) {    
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => {
            warn!("No CameraWorld found or multiple CameraWorlds detected.");
            return;
        },
    };

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let hits = raycast.cast_ray(ray, &RaycastSettings::default());
    // info!("{:?}", hits);

    // Loop through the raycast hits and detect if we hit an interactable entity
    for (entity, intersection) in hits {
        if Some(interactable_query.get(*entity)).is_some() {
            // This entity has an Interactable component
            // info!("Clicked on entity: {:?}", entity.index());

            let button_index = entity.index();

            if let Some(button) = CalcButtons::from_index(button_index) {
                button.button_info(); // Call the method to log which button was clicked
            } else {
                warn!("Unknown button index: {}", button_index);
            }
        }
    }
}
