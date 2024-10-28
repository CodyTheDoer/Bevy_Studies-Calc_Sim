use bevy::prelude::*;
use bevy::input::common_conditions::*;

use bevy_mod_raycast::prelude::*;

use calc_sim::cam_ui::setup_ui;
use calc_sim::cam_ui::CameraUi;
use calc_sim::cam_world::{pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::{CameraWorld, PanOrbitState};
use calc_sim::game_env::spawn_gltf;
use calc_sim::game_env::Interactable;

/*
Index Notes for Calc Interactions:
Calculator Body:    39
Calculator Screen:  64
Button Sum:         42
Button Add:         43
Button Subtract:    44
Button Multiply:    45
Button Divide:      46
Button Clear:       48
Button Decimal:     50
Button Num 0:       49
Button Num 1:       52
Button Num 2:       53
Button Num 3:       54
Button Num 4:       56
Button Num 5:       57
Button Num 6:       58
Button Num 7:       60
Button Num 8:       61
Button Num 9:       62
*/

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
            info!("Clicked on entity: {:?}", entity.index());
            
            // Perform your custom logic here, such as triggering an event
        }
    }
}
