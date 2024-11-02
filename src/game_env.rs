use bevy::prelude::*;
use bevy::ecs::event::EventReader;

use bevy_mod_raycast::prelude::*;

use crate::cam_world::CameraWorld;
use crate::{sum_calc_operations};
use crate::{OpIndex, SumCurrent, SumVariable};

// --- Declarations: Structs --- //

#[derive(Component)] 
pub struct ButtonAnimation {
    progress: f32,
    duration: f32,
    initial_scale: Vec3,
    target_scale: Vec3,
    target_entity: Entity,
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
pub struct Ground;

#[derive(Asset, Component, TypePath)]
pub struct Interactable; 

#[derive(Component)]
pub struct Loaded;

#[derive(Resource)]
pub struct TargetEntity {
    target_entity: u32,
}

#[derive(Resource)]
pub struct ScreenAlbedoState {
    state: u32,
}

// --- Declarations: Enums --- //

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

#[derive(Debug, Resource)]
pub enum MeshColor { // If changed update VARIANT_COUNT 
    Black,
    White,
    Red,
    Green,
    Blue,
}

// --- Declarations: Implementations (impl) --- //

impl CalcButtons {
    pub fn from_index(index: u32) -> Option<CalcButtons> {
        match index {
            45 => Some(CalcButtons::Sum),
            51 => Some(CalcButtons::Clear),
            53 => Some(CalcButtons::Decimal),
            46 => Some(CalcButtons::Add),
            47 => Some(CalcButtons::Subtract),
            48 => Some(CalcButtons::Multiply),
            49 => Some(CalcButtons::Divide),
            52 => Some(CalcButtons::Num0),
            55 => Some(CalcButtons::Num1),
            56 => Some(CalcButtons::Num2),
            57 => Some(CalcButtons::Num3),
            59 => Some(CalcButtons::Num4),
            60 => Some(CalcButtons::Num5),
            61 => Some(CalcButtons::Num6),
            63 => Some(CalcButtons::Num7),
            64 => Some(CalcButtons::Num8),
            65 => Some(CalcButtons::Num9),
            42 => Some(CalcButtons::NoneButtonBody),
            67 => Some(CalcButtons::NoneButtonScreen),
            68 => Some(CalcButtons::NoneButtonLightPanel),
            _ => None, // Handle invalid index
        }
    }

    pub fn button_info(&self) {
        info!("Button Clicked: {:?}", self);
    }
}

impl MeshColor {
    pub const VARIANT_COUNT: u32 = 4;
}

impl ScreenAlbedoState {
    pub fn new() -> Self {
        Self {
            state: 0,
        }
    }

    pub fn should_run(&self) -> bool {
        self.state == 1
    }
}

impl TargetEntity {
    pub fn new() -> Self {
        let target_entity: u32 = 0;
        TargetEntity {
            target_entity,
        }
    }
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.125, TimerMode::Once), // Set single timer for countdown
            loop_count: MeshColor::VARIANT_COUNT + 1, // +1 accounts for indexed logic
            current_count: 0,
            is_active: false,  // Initially inactive
        }
    }
}

// --- Declarations: Implementations for x --- //

impl Default for Countdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ScreenAlbedoState {
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
        if let Some(call) = CurrentMeshColor::from_index(op.screen_color) {
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
        color_change_cube_query: Query<(Entity, &Handle<Scene>), (With<Interactable>, With<Loaded>)>,
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

// --- Declarations: Functions --- //

pub fn spawn_gltf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    let gltf = ass.load("calculator.glb#Scene0");

    // Scene
    commands.spawn(SceneBundle {
        scene: gltf,
        ..Default::default()
    })
    .insert(Interactable); // Custom marker to identify this as interactable

    // Circular plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Circle::new(2000.)).into(),
            material: materials.add(Color::srgb(0.1, 0.0, 0.1)),
            transform: Transform {
                translation: Vec3::new(0.0, -0.65, 0.0),
                rotation: Quat::from_rotation_x(-2.0 * (std::f32::consts::PI / 4.0)), //4 = 45 degrees
                ..default()
            },
            ..default()
        },
        Ground,
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn fire_ray(
    mut commands: Commands,
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    windows: Query<&Window>,
    interactable_query: Query<Entity, With<Interactable>>,
    mut sum: ResMut<SumCurrent>,
    mut var: ResMut<SumVariable>,
    mut op: ResMut<OpIndex>,
    mut screen_albedo: ResMut<ScreenAlbedoState>,
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
            info!("Entity Check: {:?}", &entity);

            if let Some(button) = CalcButtons::from_index(button_index) {
                // button.button_info(); // Call the method to log which button was clicked
                match button {
                    CalcButtons::Clear => {
                        // Assuming sum has a method to reset to zero
                        // sum.zero();
                        op.index = 1;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                        // info!("Triggered button press animation for: C");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Decimal => {
                        var.decimal();
                        // info!("Triggered button press animation for: .");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Add => {
                        // Assuming there is an addition operation on `sum` involving `var`
                        // sum.add(var);
                        op.index = 2;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                        // info!("Triggered button press animation for: +");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Subtract => {
                        // sum.subtract(var);
                        op.index = 3;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                        // info!("Triggered button press animation for: -");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Multiply => {
                        // sum.multiply(var);
                        op.index = 4;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                        // info!("Triggered button press animation for: *");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Divide => {
                        // sum.divide(var);
                        op.index = 5;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                        // info!("Triggered button press animation for: /");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Sum => {
                        // var.review(); // Reviews the Vec of numbers stored in the Variable Vec and the period index.
                        op.index = 6;
                        sum_calc_operations(&mut op, &mut var, &mut sum);
                        // info!("Triggered button press animation for: =");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num0 => {
                        var.push(0);
                        // info!("Num0: {:?}", var.var);
                        // info!("Triggered button press animation for: 0");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num1 => {
                        var.push(1);
                        // info!("Num1: {:?}", var.var);
                        // info!("Triggered button press animation for: 1");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num2 => {
                        var.push(2);
                        // info!("Num2: {:?}", var.var);
                        // info!("Triggered button press animation for: 2");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num3 => {
                        var.push(3);
                        // info!("Num3: {:?}", var.var);
                        // info!("Triggered button press animation for: 3");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num4 => {
                        var.push(4);
                        // info!("Num4: {:?}", var.var);
                        // info!("Triggered button press animation for: 4");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num5 => {
                        var.push(5);
                        // info!("Num5: {:?}", var.var);
                        // info!("Triggered button press animation for: 5");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num6 => {
                        var.push(6);
                        // info!("Num6: {:?}", var.var);
                        // info!("Triggered button press animation for: 6");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num7 => {
                        var.push(7);
                        // info!("Num7: {:?}", var.var);
                        // info!("Triggered button press animation for: 7");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num8 => {
                        var.push(8);
                        // info!("Num8: {:?}", var.var);
                        // info!("Triggered button press animation for: 8");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::Num9 => {
                        var.push(9);
                        // info!("Num9: {:?}", var.var);
                        // info!("Triggered button press animation for: 9");
                        click_animation(&mut commands, *entity);
                    },
                    CalcButtons::NoneButtonBody => {
                        info!("Triggered calc shake animation for NoneButtonBody");
                    },
                    CalcButtons::NoneButtonScreen => {
                        screen_albedo.state = 1;
                        info!("Triggered calc flicker animation for NoneButtonScreen");
                    },
                    CalcButtons::NoneButtonLightPanel => {
                        info!("Triggered calc dim animation for NoneButtonLightPanel");
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

pub fn handle_asset_events(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<Scene>>,
    color_change_query: Query<(Entity, &Handle<Scene>), With<Interactable>>,
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

/// This system starts the countdown when the mouse is clicked.
pub fn update_screen_albedo(
    mut countdown: ResMut<Countdown>,
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
    mut countdown: ResMut<Countdown>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    children_query: Query<&Children>,
    material_query: Query<&Handle<StandardMaterial>>,
    color_change_cube_query: Query<(Entity, &Handle<Scene>), (With<Interactable>, With<Loaded>)>,
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

pub fn button_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut ButtonAnimation)>,
    mut commands: Commands,
) {
    for (mut transform, mut animation) in query.iter_mut() {
        animation.progress += time.delta_seconds();

        let factor = (animation.progress / animation.duration).min(1.0);
        transform.scale = animation.initial_scale.lerp(animation.target_scale, factor);

        if factor >= 1.0 {
            // Remove the animation component once the animation is complete
            commands.entity(animation.target_entity).remove::<ButtonAnimation>();
        }
    }
}

pub fn click_animation(
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity).insert(ButtonAnimation {
        progress: 0.0,
        duration: 0.1,
        initial_scale: Vec3::ONE,
        target_scale: Vec3::new(1.0, 0.88, 1.0),
        target_entity: entity, // Use the current entity ID
    });
    commands.entity(entity).insert(ButtonAnimation {
        progress: 0.0,
        duration: 0.18,
        initial_scale: Vec3::new(1.0, 0.88, 1.0),
        target_scale: Vec3::ONE,
        target_entity: entity, // Use the current entity ID
    });
}