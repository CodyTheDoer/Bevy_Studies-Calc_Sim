use bevy::prelude::*;
use bevy::input::common_conditions::*;

use bevy_mod_raycast::prelude::*;

// use calc_sim::{add, subtract, multiply, divide};
use calc_sim::{FlexInput, OpIndex, SumCurrent, SumVariable};

use calc_sim::cam_ui::{setup_ui, update_sum_text, update_var_text};
use calc_sim::cam_ui::CameraUi;

use calc_sim::cam_world::{draw_cursor, pan_orbit_camera, spawn_3d_camera};
use calc_sim::cam_world::{CameraWorld, PanOrbitState};

use calc_sim::game_env::{button_animation_system, fire_ray, spawn_gltf};
use calc_sim::game_env::{CalcButtons, ColorChange, Interactable};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CurrentMeshColor>()
        .init_resource::<Countdown>()
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
                pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
                fire_ray.run_if(input_just_released(MouseButton::Left)),

                handle_asset_events,
                screen_albedo, 
                update_screen_albedo.run_if(input_just_released(MouseButton::Right)),
            )
        )
        .run();
}






















#[derive(Debug, Resource)]
pub enum MeshColor { // If changed update VARIANT_COUNT 
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl MeshColor {
    pub const VARIANT_COUNT: u32 = 4;
}

#[derive(Resource)]
pub struct Countdown {
    pub timer: Timer,           // Set single timer for countdown
    pub loop_count: u32,        // Number of loops, currently tied to the varient_count to loop through all dynamically
    pub current_count: u32,     // Tracks where in the loop you are
    pub is_active: bool,        // Tracks if the loop is active
}

#[derive(Default, Resource)]
pub struct CurrentMeshColor;

#[derive(Component)]
pub struct Loaded;

impl Countdown {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / 3.0, TimerMode::Once), // Set single timer for countdown
            loop_count: MeshColor::VARIANT_COUNT + 1, // +1 accounts for indexed logic
            current_count: 0,
            is_active: false,  // Initially inactive
        }
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentMeshColor {
    fn from_index(index: u32) -> Option<MeshColor> {
        match index {
            0 => Some(MeshColor::Black),
            1 => Some(MeshColor::White),
            2 => Some(MeshColor::Red),
            3 => Some(MeshColor::Green),
            4 => Some(MeshColor::Blue),
            _ => None, // Handle invalid index
        }
    }

    fn update_current_mesh_color(
        op: &mut ResMut<OpIndex>,
    ) -> Color {
        if let Some(call) = CurrentMeshColor::from_index(op.index) {
            match call {
                MeshColor::Black => {
                    Color::srgb(0.0, 0.0, 0.0)
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
            }
        } else {
            Color::srgb(0.0, 0.0, 0.0)
        }
    }

    fn update_gltf_material_color(
        children_query: Query<&Children>,
        color_change_cube_query: Query<(Entity, &Handle<Scene>), (With<ColorChange>, With<Loaded>)>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        material_query: Query<&Handle<StandardMaterial>>,
        op_index: &mut ResMut<OpIndex>,
    ) {
        for (entity, _) in color_change_cube_query.iter() {
            if let Ok(children) = children_query.get(entity) {
                Self::process_entity_children(
                    &mut materials,
                    &material_query,
                    children,
                    &children_query,
                    op_index,         
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
    ) {
        for &child in children.iter() {
            if child.index() == 67 { // This targets the screen component specifically, still learning about glb files and how to extract names s I don't have a more dynamic way of handling it for now.
                if let Ok(material_handle) = material_query.get(child) {
                    if let Some(material) = materials.get_mut(material_handle) {
                        material.base_color = CurrentMeshColor::update_current_mesh_color(op_index);
                    }
                }
            }
            // Recursively check grandchildren
            if let Ok(grandchildren) = children_query.get(child) {
                Self::process_entity_children(
                    materials,
                    material_query,
                    grandchildren,
                    children_query,
                    op_index,
                );
            }
        }
    }
}

/// This system starts the countdown when the mouse is clicked.
pub fn update_screen_albedo(
    mut countdown: ResMut<Countdown>,
 ) {
    // Only start the countdown if it's not already active
    if !countdown.is_active {
        countdown.is_active = true;
        countdown.current_count = 0; // Reset the current count
        countdown.timer.reset();  // Reset the timer to start fresh
    }
}

/// This system controls ticking the timer within the countdown resource and
/// handling its state.
pub fn screen_albedo(
    time: Res<Time>, 
    mut countdown: ResMut<Countdown>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    children_query: Query<&Children>,
    material_query: Query<&Handle<StandardMaterial>>,
    color_change_cube_query: Query<(Entity, &Handle<Scene>), (With<ColorChange>, With<Loaded>)>,
    mut op_index: ResMut<OpIndex>,
) {
    // Only tick the timer if the countdown is active
    if countdown.is_active {
        // Tick the timer
        countdown.timer.tick(time.delta());

        // Check if the timer has finished for the current iteration
        if countdown.timer.finished() {
            // Update the albedo before we cycle color
            CurrentMeshColor::update_gltf_material_color(
                children_query,
                color_change_cube_query,
                materials,
                material_query,
                &mut op_index,
            );

            countdown.current_count += 1;
            let color_count = MeshColor::VARIANT_COUNT;
            if op_index.index == color_count {
                op_index.index = 0;
            } else {
                op_index.index += 1;
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

pub fn handle_asset_events(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<Scene>>,
    color_change_query: Query<(Entity, &Handle<Scene>), With<ColorChange>>,
) {
    for event in events.read() {
        if let AssetEvent::Added { id } = event {
            for (entity, scene_handle) in color_change_query.iter() {
                if *id == scene_handle.id() {
                    commands.entity(entity).insert(Loaded);
                }
            }
        }
    }
}