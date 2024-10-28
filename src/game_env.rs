use bevy::prelude::*;
use bevy::ecs::event::EventReader;

use bevy_mod_raycast::prelude::*;

use crate::cam_world::CameraWorld;

#[derive(Component)]
pub struct Interactable; 

pub fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let gltf = ass.load("calculator.glb#Scene0");

    // Scene
    commands.spawn(SceneBundle {
        scene: gltf,
        ..Default::default()
    })
    .insert(Interactable); // Custom marker to identify this as interactable
    
    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Debug)]
pub enum CalcButtons {
    Sum,
    Clear,
    Decimal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    NoneButton_Body,
    NoneButton_Screen,
    NoneButton_LightPanel,
}

impl CalcButtons {
    pub fn from_index(index: u32) -> Option<CalcButtons> {
        match index {
            42 => Some(CalcButtons::Sum),
            48 => Some(CalcButtons::Clear),
            50 => Some(CalcButtons::Decimal),
            43 => Some(CalcButtons::Add),
            44 => Some(CalcButtons::Subtract),
            45 => Some(CalcButtons::Multiply),
            46 => Some(CalcButtons::Divide),
            49 => Some(CalcButtons::Num0),
            52 => Some(CalcButtons::Num1),
            53 => Some(CalcButtons::Num2),
            54 => Some(CalcButtons::Num3),
            56 => Some(CalcButtons::Num4),
            57 => Some(CalcButtons::Num5),
            58 => Some(CalcButtons::Num6),
            60 => Some(CalcButtons::Num7),
            61 => Some(CalcButtons::Num8),
            62 => Some(CalcButtons::Num9),
            39 => Some(CalcButtons::NoneButton_Body),
            64 => Some(CalcButtons::NoneButton_Screen),
            65 => Some(CalcButtons::NoneButton_LightPanel),
            _ => None, // Handle invalid index
        }
    }

    pub fn button_info(&self) {
        info!("Button Clicked: {:?}", self);
    }
}

pub fn fire_ray(
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
