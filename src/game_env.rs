use bevy::prelude::*;
use bevy::ecs::event::EventReader;

use bevy_mod_raycast::prelude::*;

use crate::cam_world::CameraWorld;
use crate::sum_calc_operations;
use crate::{OpIndex, SumCurrent, SumVariable};

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
    NoneButtonBody,
    NoneButtonScreen,
    NoneButtonLightPanel,
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
            39 => Some(CalcButtons::NoneButtonBody),
            64 => Some(CalcButtons::NoneButtonScreen),
            65 => Some(CalcButtons::NoneButtonLightPanel),
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
    mut sum: ResMut<SumCurrent>,
    mut var: ResMut<SumVariable>,
    mut op: ResMut<OpIndex>,
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
                // button.button_info(); // Call the method to log which button was clicked
                match button {
                    CalcButtons::Sum => {
                        // info!("sum: {}", sum.sum);
                        // info!("var: {:?}", var.var);
                        // info!("decimal index: {}", var.decimal_index);
                        op.index = 6;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                        // info!("Sum: {:?}", &mut sum.sum);
                    },
                    CalcButtons::Clear => {
                        // Assuming sum has a method to reset to zero
                        // sum.zero();
                        op.index = 1;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                        var.review()
                    },
                    CalcButtons::Decimal => {
                        var.decimal();
                    },
                    CalcButtons::Add => {
                        // Assuming there is an addition operation on `sum` involving `var`
                        // sum.add(var);
                        op.index = 2;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                    },
                    CalcButtons::Subtract => {
                        // sum.subtract(var);
                        op.index = 3;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                    },
                    CalcButtons::Multiply => {
                        // sum.multiply(var);
                        op.index = 4;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                    },
                    CalcButtons::Divide => {
                        // sum.divide(var);
                        op.index = 5;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                    },
                    CalcButtons::Num0 => {
                        var.push(0);
                    },
                    CalcButtons::Num1 => {
                        var.push(1);
                    },
                    CalcButtons::Num2 => {
                        var.push(2);
                    },
                    CalcButtons::Num3 => {
                        var.push(3);
                    },
                    CalcButtons::Num4 => {
                        var.push(4);
                    },
                    CalcButtons::Num5 => {
                        var.push(5);
                    },
                    CalcButtons::Num6 => {
                        var.push(6);
                    },
                    CalcButtons::Num7 => {
                        var.push(7);
                    },
                    CalcButtons::Num8 => {
                        var.push(8);
                    },
                    CalcButtons::Num9 => {
                        var.push(9);
                    },
                    CalcButtons::NoneButtonBody => {
                        info!("NoneButtonBody");
                    },
                    CalcButtons::NoneButtonScreen => {
                        info!("NoneButtonScreen");
                    },
                    CalcButtons::NoneButtonLightPanel => {
                        info!("NoneButtonLightPanel");
                    },
                    _ => {
                        // Handle invalid button case, if needed
                        info!("Invalid button press");
                    },
                }
            } 
        }
    }
}