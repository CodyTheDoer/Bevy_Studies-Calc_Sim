use bevy::prelude::*;

use crate::OpIndex;
use crate::cam_calc_screen::CalcUIMaterialHandle;
use crate::game_env::{CountdownCycle, Interactable, Loaded};

use std::collections::HashMap;

/// This system starts the countdown when the mouse is clicked.
pub fn cycle_screen_albedo(
    mut countdown: ResMut<CountdownCycle>,
    mut screen_albedo_state: ResMut<ScreenAlbedoState>,
 ) {
    if !countdown.is_active {
        countdown.is_active = true;
        countdown.current_count = 0; // Reset the current count
        countdown.timer.reset();  // Reset the timer to start fresh
    }
    screen_albedo_state.state = 0;
}

/// This system controls ticking the timer within the countdown resource and
/// handling its state.
pub fn screen_albedo(
    time: Res<Time>, 
    mut countdown: ResMut<CountdownCycle>,
    materials: ResMut<Assets<StandardMaterial>>,
    children_query: Query<&Children>,
    material_query: Query<&Handle<StandardMaterial>>,
    color_change_query: Query<(Entity, &Handle<Scene>), (With<Interactable>, With<Loaded>)>,
    mut op_index: ResMut<OpIndex>,
    mut calc_ui_material: ResMut<CalcUIMaterialHandle>,
) {
    // Update the albedo before we cycle color
    CurrentMeshColor::update_gltf_material_color(
        children_query,
        color_change_query,
        materials,
        material_query,
        &mut op_index,
        &mut calc_ui_material,
    );
    
    // Only tick the timer if the countdown is active
    if countdown.is_active {
        // Tick the timer
        countdown.timer.tick(time.delta());

        // Check if the timer has finished for the current iteration
        if countdown.timer.finished() {

            countdown.current_count += 1;
            let color_count = MeshColor::VARIANT_COUNT;
            if op_index.screen_color >= color_count {
                op_index.screen_color = 0;
            } else {
                op_index.screen_color += 1;
            }
            // If we've completed all iterations, stop the countdown
            if countdown.current_count >= countdown.loop_count {
                countdown.is_active = false;
            } else {
                // Otherwise, reset the timer for the next iteration
                countdown.timer.reset();
            }
        } 
    }
}

#[derive(Default, Resource)]
pub struct CurrentMeshColor;

impl CurrentMeshColor {
    fn from_index(index: u32) -> Option<MeshColor> {
        match index {
            0 => Some(MeshColor::Gray),
            1 => Some(MeshColor::White),
            2 => Some(MeshColor::Red),
            3 => Some(MeshColor::Green),
            4 => Some(MeshColor::Blue),
            5 => Some(MeshColor::Black),
            _ => None, // Handle invalid index
        }
    }

    fn update_current_mesh_color(
        op: &mut ResMut<OpIndex>,
    ) -> Color {
        if let Some(call) = CurrentMeshColor::from_index(op.screen_color) {
            match call {
                MeshColor::Gray => {
                    Color::srgb(0.5, 0.5, 0.5)
                },
                MeshColor::White => {
                    Color::srgb(1.0, 1.0, 1.0)
                },
                MeshColor::Red => {
                    Color::srgb(1.0, 0.0, 0.0)
                },
                MeshColor::Green => {
                    Color::srgb(0.0, 1.0, 0.0)
                },
                MeshColor::Blue => {
                    Color::srgb(0.0, 0.0, 1.0)
                },
                MeshColor::Black => {
                    Color::srgb(0.0, 0.0, 0.0)
                },
            }
        } else {
            Color::srgb(0.0, 0.0, 0.0)
        }
    }

    fn update_gltf_material_color(
        children_query: Query<&Children>,
        color_change_cube_query: Query<(Entity, &Handle<Scene>), (With<Interactable>, With<Loaded>)>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        material_query: Query<&Handle<StandardMaterial>>,
        op_index: &mut ResMut<OpIndex>,
        calc_ui_material: &mut ResMut<CalcUIMaterialHandle>,
    ) {
        for (entity, _) in color_change_cube_query.iter() {
            if let Ok(children) = children_query.get(entity) {
                Self::process_entity_children(
                    &mut materials,
                    &material_query,
                    children,
                    &children_query,
                    op_index,      
                    calc_ui_material,   
                );
            }
        }
    }

    fn process_entity_children(
        materials: &mut ResMut<Assets<StandardMaterial>>,
        material_query: &Query<&Handle<StandardMaterial>>,
        children: &Children,
        children_query: &Query<&Children>,
        op_index: &mut ResMut<OpIndex>,
        calc_ui_material: &mut ResMut<CalcUIMaterialHandle>,
    ) {
        for &child in children.iter() {
            if child.index() == 62 + op_index.entities {
                if let Ok(material_handle) = material_query.get(child) {
                    if let Some(material) = materials.get_mut(material_handle) {
                        material.base_color = CurrentMeshColor::update_current_mesh_color(op_index);
                        material.base_color_texture = Some(calc_ui_material.image_handle.clone());
                    } else {
                        warn!("Material not found or invalid for handle: {:?}", material_handle);                    }
                } else {
                    warn!("Could not get material handle for child: {:?}", child);                }
            } else {
            }

            // Recursively check grandchildren
            if let Ok(grandchildren) = children_query.get(child) {
                Self::process_entity_children(
                    materials,
                    material_query,
                    grandchildren,
                    children_query,
                    op_index,
                    calc_ui_material,
                );
            }
        }
    }
}

#[derive(Debug, Resource)]
pub enum MeshColor { // If changed update VARIANT_COUNT 
    Gray,
    White,
    Red,
    Green,
    Blue,
    Black,
}

impl MeshColor {
    pub const VARIANT_COUNT: u32 = 4; 
    // The VARIENT_COUNT is indexed logic.
    // black is not accounted for here, as it is not an active display color.
}

#[derive(Resource)]
pub struct ScreenAlbedoState {
    pub state: u32,
}

impl ScreenAlbedoState {
    pub fn new() -> Self {
        Self {
            state: 0,
        }
    }

    pub fn should_run_cycle(&self) -> bool {
        self.state == 1
    }

    pub fn should_run_dim(&self) -> bool {
        self.state == 2
    }
}

impl Default for ScreenAlbedoState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
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
    pub fn from_index(
        op_index: &mut ResMut<OpIndex>,
        index: u32,
    ) -> Option<CalcButtons> {
        let mut button_map = HashMap::new();

        button_map.insert(40 + op_index.entities, CalcButtons::Sum);
        button_map.insert(46 + op_index.entities, CalcButtons::Clear);
        button_map.insert(48 + op_index.entities, CalcButtons::Decimal);
        button_map.insert(41 + op_index.entities, CalcButtons::Add);
        button_map.insert(42 + op_index.entities, CalcButtons::Subtract);
        button_map.insert(43 + op_index.entities, CalcButtons::Multiply);
        button_map.insert(44 + op_index.entities, CalcButtons::Divide);
        button_map.insert(47 + op_index.entities, CalcButtons::Num0);
        button_map.insert(50 + op_index.entities, CalcButtons::Num1);
        button_map.insert(51 + op_index.entities, CalcButtons::Num2);
        button_map.insert(52 + op_index.entities, CalcButtons::Num3);
        button_map.insert(54 + op_index.entities, CalcButtons::Num4);
        button_map.insert(55 + op_index.entities, CalcButtons::Num5);
        button_map.insert(56 + op_index.entities, CalcButtons::Num6);
        button_map.insert(58 + op_index.entities, CalcButtons::Num7);
        button_map.insert(59 + op_index.entities, CalcButtons::Num8);
        button_map.insert(60 + op_index.entities, CalcButtons::Num9);
        button_map.insert(37 + op_index.entities, CalcButtons::NoneButtonBody);
        button_map.insert(62 + op_index.entities, CalcButtons::NoneButtonScreen);
        button_map.insert(63 + op_index.entities, CalcButtons::NoneButtonLightPanel);
    
        button_map.get(&index).cloned()
    }

    pub fn button_info(&self) {
        info!("Button Clicked: {:?}", self);
    }
}