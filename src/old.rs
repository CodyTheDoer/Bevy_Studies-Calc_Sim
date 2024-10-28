use bevy::prelude::*;
use bevy::input::common_conditions::*;
use bevy::reflect::TypePath;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::Indices;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::*;

use bevy::{
    color::palettes::css::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<MyMeshHandle>()
        .init_resource::<Triangles>()
        .add_systems(Startup, setup)
        .add_systems(Startup, load_gltf_extract_triangles)
        // .add_systems(Startup, spawn_gltf)
        .add_systems(Startup, spawn_3d_camera)
        .add_systems(Update, spawn_gltf)
        .add_systems(Update,
            pan_orbit_camera.run_if(any_with_component::<PanOrbitState>))
        .add_systems(Update,
            extract_triangles.run_if(is_triangle_count_zero))
        .add_systems(Update,
            print_triangle_count.run_if(input_pressed(MouseButton::Left)))
        .run();
}



fn load_gltf_extract_triangles(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    info!("Loading GLTF scene and extracting meshes...");

    // Load the main GLTF scene.
    let my_gltf: Handle<Scene> = ass.load("calculator.glb#Scene0");

    // Load the GLTF file and parse its meshes dynamically.
    for i in 0..27 { // Pick safe upper bound for the number of meshes
        let mesh_path = format!("calculator.glb#Mesh{}/Primitive0", i);
        let mesh_handle: Handle<Mesh> = ass.load(mesh_path.clone());

        // If mesh exists, insert the mesh handle as a resource for future use.
        commands.insert_resource(MyMeshHandle(mesh_handle.clone()));
        info!("Loaded mesh path {}: {}", i, mesh_path);
    }

    // Spawn the GLTF scene using SceneBundle, similar to the `spawn_gltf` function.
    commands.spawn(SceneBundle {
        scene: my_gltf,
        ..Default::default()
    });
}

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // Load the main GLTF scene and spawn it in the world.
    let my_gltf = ass.load("calculator.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: my_gltf,
        ..Default::default()
    });
}

fn extract_mesh_triangles(meshes: &Res<Assets<Mesh>>, mesh_handle: &Handle<Mesh>) {
    if let Some(mesh) = meshes.get(mesh_handle) {
        if let Some(vertex_indices) = mesh.indices() {
            match vertex_indices {
                bevy::render::mesh::Indices::U16(indices) => {
                    for triangle in indices.chunks(3) {
                        info!("Triangle (u16): {:?}", triangle);
                    }
                }
                bevy::render::mesh::Indices::U32(indices) => {
                    for triangle in indices.chunks(3) {
                        info!("Triangle (u32): {:?}", triangle);
                    }
                }
            }
        }
    }
}






struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
}

impl Triangle {
    fn new(
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
    ) -> Self {
        Triangle {
            p1,
            p2,
            p3,
        }
    }

    fn p1(&self) -> Vec3 {
        self.p1
    }

    fn p2(&self) -> Vec3 {
        self.p2
    }
    
    fn p3(&self) -> Vec3 {
        self.p3
    }
}

#[derive(Default, Resource)]
struct Triangles {
    triangles: Vec<Triangle>,
}

impl Triangles {
    fn new() -> Self {
        let triangles: Vec<Triangle> = Vec::new();
        Triangles{
            triangles,
        }
    }

    fn new_from_tri(triangle: Triangle) -> Self {
        let mut triangles: Vec<Triangle> = Vec::new();
        triangles.push(triangle);
        Triangles{
            triangles,
        }
    }
}

fn print_triangle_count(
    triangles: Res<Triangles>,
) {
    let mut count: u32 = 0; 
    for triangle in triangles.triangles.iter() {
        count += 1;
    }
    println!("{:?}", count);
}

fn is_triangle_count_zero(triangles: Res<Triangles>) -> bool {
    triangles.triangles.is_empty()
}

#[derive(Default, Resource)]
struct MyMeshHandle(Handle<Mesh>);

// fn load_gltf_extract_triangles(
//     mut commands: Commands,
//     ass: Res<AssetServer>,
//     meshes: Res<Assets<Mesh>>,
//     mesh_handle: Res<MyMeshHandle>,
// ) {
//     info!("load_gltf before");
//     let meshes_to_try = vec![
//         // "calculator.glb#Mesh0/Primitive0".to_string(),   // 901  Triangles
//         // "calculator.glb#Mesh1/Primitive0".to_string(),   // 218  Triangles
//         // "calculator.glb#Mesh2/Primitive0".to_string(),   // 1010 Triangles
//         // "calculator.glb#Mesh3/Primitive0".to_string(),   // 1010 Triangles
//         // "calculator.glb#Mesh4/Primitive0".to_string(),   // 8    Triangles
//         "calculator.glb#Mesh5/Primitive0".to_string(),   // 8    Triangles
//         // "calculator.glb#Mesh6/Primitive0".to_string(),   // 8    Triangles
//         // "calculator.glb#Mesh7/Primitive0".to_string(),   // 8    Triangles
//         // "calculator.glb#Mesh8/Primitive0".to_string(),   // 8    Triangles
//         // "calculator.glb#Mesh9/Primitive0".to_string(),   // 334  Triangles
//         // "calculator.glb#Mesh10/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh11/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh12/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh13/Primitive0".to_string(),  // 325  Triangles
//         // "calculator.glb#Mesh14/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh15/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh16/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh17/Primitive0".to_string(),  // 324  Triangles
//         // "calculator.glb#Mesh18/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh19/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh20/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh21/Primitive0".to_string(),  // 290  Triangles
//         // "calculator.glb#Mesh22/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh23/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh24/Primitive0".to_string(),  // 8    Triangles
//         // "calculator.glb#Mesh25/Primitive0".to_string(),  // 445  Triangles
//         // "calculator.glb#Mesh26/Primitive0".to_string(),  // 1    Triangle
//         // "calculator.glb#Mesh27/Primitive0".to_string(),  // 1    Triangle
//         // "calculator.glb#Mesh28/Primitive0".to_string(),  // 12   Triangles Button
//     ];

//     for (i, mesh_path) in meshes_to_try.iter().enumerate() {
//         let mesh_handle: Handle<Mesh> = ass.load(mesh_path);
//         commands.insert_resource(MyMeshHandle(mesh_handle.clone()));
//         info!("Loaded mesh path {}: {}", i, mesh_path);
//     }
// }

// fn load_gltf(
//     mut commands: Commands,
//     ass: Res<AssetServer>,
// ) {
//     info!("load_gltf before");
//     let meshes_to_try = vec![
//         "calculator.glb#Mesh0/Primitive0".to_string(),
//         "calculator.glb#Mesh1/Primitive0".to_string(),
//         "calculator.glb#Mesh2/Primitive0".to_string(),
//         "calculator.glb#Mesh3/Primitive0".to_string(),
//         "calculator.glb#Mesh4/Primitive0".to_string(),
//         "calculator.glb#Mesh5/Primitive0".to_string(),
//     ];

//     for (i, mesh_path) in meshes_to_try.iter().enumerate() {
//         let mesh_handle: Handle<Mesh> = ass.load(mesh_path);
//         commands.insert_resource(MyMeshHandle(mesh_handle.clone()));
//         // commands.insert_resource(MeshLoadTracker { loaded: false });
//         info!("Loaded mesh path {}: {}", i, mesh_path);
//     }
// }

fn extract_triangles(
    mesh_handle: Res<MyMeshHandle>,
    meshes: Res<Assets<Mesh>>,
    mut triangles: ResMut<Triangles>,
) {
    
    if let Some(mesh) = meshes.get(&mesh_handle.0) {
        info!("mesh check");
        // Get the Vertex positions
        if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            info!("VertexAttributeValues check");
            // Get the Indices if available
            if let Some(Indices::U32(indices)) = mesh.indices() {
                info!("Indices check");
                // Extract the Triangles from the Indices
                for triangle in indices.chunks(3) {
                    if let [i1, i2, i3] = *triangle {
                        let v1 = positions[i1 as usize];
                        let v2 = positions[i2 as usize];
                        let v3 = positions[i3 as usize];
                        info!("Triangle: {:?}, {:?}, {:?}", v1, v2, v3);
                        triangles.triangles.push(Triangle::new(v1.into(), v2.into(), v3.into()));
                    }
                } 
            } else {
                // If no idices, treat the vertices as individual triangles
                info!("Vertices check");
                for triangle in positions.chunks(3) {
                    if let [v1, v2, v3] = *triangle {
                        info!("Triangle: {:?}, {:?}, {:?}", v1, v2, v3);
                        triangles.triangles.push(Triangle::new(v1.into(), v2.into(), v3.into()));
                    }
                }
            }
        } else {
            info!("Mesh has no POSITION attribute");
        }
    } else {
        // info!("Mesh not yet loaded, trying again...");
    }   
}

// fn spawn_gltf(
//     mut commands: Commands,
//     ass: Res<AssetServer>,
//     meshes: Res<Assets<Mesh>>,
//     mesh_handle: Res<MyMeshHandle>,
// ) {
//     // note that we have to include the `Scene0` label
//     let my_gltf = ass.load("calculator.glb#Scene0");

//     // to position our 3d model, simply use the Transform
//     // in the SceneBundle
//     commands.spawn(SceneBundle {
//         scene: my_gltf,
//         ..Default::default()
//     });
// }

// Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: PanOrbitState,
    pub settings: PanOrbitSettings,
}

// The internal state of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

/// The configuration of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,
    /// Radians per pixel of mouse motion
    pub orbit_sensitivity: f32,
    /// Exponent per pixel of mouse motion
    pub zoom_sensitivity: f32,
    /// Key to hold for panning
    pub pan_key: Option<KeyCode>,
    /// Key to hold for orbiting
    pub orbit_key: Option<KeyCode>,
    /// Key to hold for zooming
    pub zoom_key: Option<KeyCode>,
    /// What action is bound to the scroll wheel?
    pub scroll_action: Option<PanOrbitAction>,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanOrbitAction {
    Pan,
    Orbit,
    Zoom,
}

impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        PanOrbitSettings {
            pan_sensitivity: 0.001, // 1000 pixels per world unit
            orbit_sensitivity: 0.1f32.to_radians(), // 0.1 degree per pixel
            zoom_sensitivity: 0.01,
            pan_key: Some(KeyCode::ControlLeft),
            orbit_key: Some(KeyCode::AltLeft),
            zoom_key: Some(KeyCode::ShiftLeft),
            scroll_action: Some(PanOrbitAction::Zoom),
            scroll_line_sensitivity: 16.0, // 1 "line" == 16 "pixels of motion"
            scroll_pixel_sensitivity: 1.0,
        }
    }
}

use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};

use std::f32::consts::{FRAC_PI_2, PI, TAU};

fn pan_orbit_camera(
    kbd: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camera: Query<(
        &PanOrbitSettings,
        &mut PanOrbitState,
        &mut Transform,
    )>,
) {
    // First, accumulate the total amount of
    // mouse motion and scroll, from all pending events:
    let mut total_motion: Vec2 = evr_motion.read()
        .map(|ev| ev.delta).sum();

    // Reverse Y (Bevy's Worldspace coordinate system is Y-Up,
    // but events are in window/ui coordinates, which are Y-Down)
    total_motion.y = -total_motion.y;

    let mut total_scroll_lines = Vec2::ZERO;
    let mut total_scroll_pixels = Vec2::ZERO;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                total_scroll_lines.x += ev.x;
                total_scroll_lines.y -= ev.y;
            }
            MouseScrollUnit::Pixel => {
                total_scroll_pixels.x += ev.x;
                total_scroll_pixels.y -= ev.y;
            }
        }
    }

    for (settings, mut state, mut transform) in &mut q_camera {
        // Check how much of each thing we need to apply.
        // Accumulate values from motion and scroll,
        // based on our configuration settings.

        let mut total_pan = Vec2::ZERO;
        if settings.pan_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_pan -= total_motion * settings.pan_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Pan) {
            total_pan -= total_scroll_lines
                * settings.scroll_line_sensitivity * settings.pan_sensitivity;
            total_pan -= total_scroll_pixels
                * settings.scroll_pixel_sensitivity * settings.pan_sensitivity;
        }

        let mut total_orbit = Vec2::ZERO;
        if settings.orbit_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_orbit -= total_motion * settings.orbit_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Orbit) {
            total_orbit -= total_scroll_lines
                * settings.scroll_line_sensitivity * settings.orbit_sensitivity;
            total_orbit -= total_scroll_pixels
                * settings.scroll_pixel_sensitivity * settings.orbit_sensitivity;
        }

        let mut total_zoom = Vec2::ZERO;
        if settings.zoom_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_zoom -= total_motion * settings.zoom_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Zoom) {
            total_zoom -= total_scroll_lines
                * settings.scroll_line_sensitivity * settings.zoom_sensitivity;
            total_zoom -= total_scroll_pixels
                * settings.scroll_pixel_sensitivity * settings.zoom_sensitivity;
        }

        // Upon starting a new orbit maneuver (key is just pressed),
        // check if we are starting it upside-down
        if settings.orbit_key.map(|key| kbd.just_pressed(key)).unwrap_or(false) {
            state.upside_down = state.pitch < -FRAC_PI_2 || state.pitch > FRAC_PI_2;
        }

        // If we are upside down, reverse the X orbiting
        if state.upside_down {
            total_orbit.x = -total_orbit.x;
        }

        // Now we can actually do the things!

        let mut any = false;

        // To ZOOM, we need to multiply our radius.
        if total_zoom != Vec2::ZERO {
            any = true;
            // in order for zoom to feel intuitive,
            // everything needs to be exponential
            // (done via multiplication)
            // not linear
            // (done via addition)

            // so we compute the exponential of our
            // accumulated value and multiply by that
            state.radius *= (-total_zoom.y).exp();
        }

        // To ORBIT, we change our pitch and yaw values
        if total_orbit != Vec2::ZERO {
            any = true;
            state.yaw += total_orbit.x;
            state.pitch += total_orbit.y;
            // wrap around, to stay between +- 180 degrees
            if state.yaw > PI {
                state.yaw -= TAU; // 2 * PI
            }
            if state.yaw < -PI {
                state.yaw += TAU; // 2 * PI
            }
            if state.pitch > PI {
                state.pitch += TAU; // 2 * PI
            }
            if state.pitch < -PI {
                state.pitch -= TAU; // 2 * PI
            }
        }

        // To PAN, we can get the UP and RIGHT direction
        // vectors from the camera's transform, and use
        // them to move the center point. Multiply by the
        // radius to make the pan adapt to the current zoom.
        if total_pan != Vec2::ZERO {
            any = true;
            let radius = state.radius;
            state.center += transform.right() * total_pan.x * radius;
            state.center += transform.up() * total_pan.y * radius;
        }

        // Finally, compute the new camera transform.
        // (if we changed anything, or if the pan-orbit
        // controller was just added and thus we are running
        // for the first time and need to initialize)
        if any || state.is_added() {
            // YXZ Euler Rotation performs yaw/pitch/roll.
            transform.rotation =
                Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
            // To position the camera, get the backward direction vector
            // and place the camera at the desired radius from the center.
            transform.translation = state.center + transform.back() * state.radius;
        }
    }
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

fn spawn_3d_camera(mut commands: Commands) {
    let mut camera = PanOrbitCameraBundle::default();
    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(0.0, 0.0, 0.0);
    camera.state.radius = 20.0;
    camera.state.pitch = -30.0f32.to_radians();
    camera.state.yaw = 0.0f32.to_radians();
    commands.spawn(camera);
}

// UI with direct spawn
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/MatrixtypeDisplay-KVELZ.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 42.0,
        ..default()
    };
    let smaller_text_style = TextStyle {
        font: font.clone(),
        font_size: 25.0,
        ..default()
    };

    // UI Camera (2D)
    let mut my_2d_camera_bundle = Camera2dBundle {
        camera: Camera {
            order: 1, // Render on top of the 3D scene
            ..default()
        },
        ..default()
    };
    commands.spawn(my_2d_camera_bundle);

    // Create a screen-sized UI node as a container
    commands.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            align_items: AlignItems::Center,    // Center vertically within the container
            justify_content: JustifyContent::Center, // Center horizontally within the container
            position_type: PositionType::Absolute,
            // Set this node to occupy the entire screen
            width: Val::Percent(100.0),   // Use width instead of size
            height: Val::Percent(100.0),  // Use height instead of size
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Calc-Sim...",
                    text_style,
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                // Manually set the position of the text to the bottom left
                // align_self: AlignSelf::Center, // Center horizontally relative to its own width
                top: Val::Percent(2.0),  // 10 pixels from the top
                // left: Val::Percent(50.0),  // 50% from the left
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "SHIFT: Zoom\nCTRL: Pan\nL-ALT: Rotate",
                    smaller_text_style,
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                // Manually set the position of the text to the bottom left
                left: Val::Percent(1.5),  // 10 pixels from the left
                bottom: Val::Percent(2.0),  // 10 pixels from the bottom
                ..default()
            },
            ..default()
        });
    });
}