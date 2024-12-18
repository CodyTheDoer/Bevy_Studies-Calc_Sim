use bevy::prelude::*;
use bevy::ecs::event::EventReader;

use bevy_mod_raycast::prelude::*;

use crate::calculator::{CalcButtons, MeshColor, ScreenAlbedoState};
use crate::cam_world::CameraWorld;
use crate::{sum_calc_operations};
use crate::{OpIndex, SumCurrent, SumVariable};

pub fn fire_ray(
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    windows: Query<&Window>,
    interactable_query: Query<Entity, With<Interactable>>,
    mut op_index: ResMut<OpIndex>,
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

    // Loop through the raycast hits and detect if we hit an interactable entity
    for (entity, _intersection) in hits {
        if Some(interactable_query.get(*entity)).is_some() {
            let button_index = entity.index();
            if let Some(button) = CalcButtons::from_index(&mut op_index, button_index) {
                match button {
                    CalcButtons::NoneButtonLightPanel => {
                        screen_albedo.state = 2;
                    },
                    _ => {
                    },
                }
            } 
        }
    }
}

pub fn release_ray(
    mut commands: Commands,
    mut raycast: Raycast,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraWorld>>, // Only query for the CameraWorld    
    windows: Query<&Window>,
    interactable_query: Query<Entity, With<Interactable>>,
    mut sum: ResMut<SumCurrent>,
    mut var: ResMut<SumVariable>,
    mut op_index: ResMut<OpIndex>,
    mut screen_albedo: ResMut<ScreenAlbedoState>,
    asset_server: Res<AssetServer>,
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

    // Loop through the raycast hits and detect if we hit an interactable entity
    for (entity, _intersection) in hits {
        if Some(interactable_query.get(*entity)).is_some() {
            let button_index = entity.index();
            if let Some(button) = CalcButtons::from_index(&mut op_index, button_index) {
                match button {
                    CalcButtons::Clear => {
                        op_index.index = 1;
                        op_index.last_op = 0;
                        sum_calc_operations(&mut op_index, &mut var, &mut sum);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Decimal => {                        
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        if var.var.len() == 0 {
                            var.push(0);
                        }
                        var.decimal();
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Add => {
                        op_index.index = 2;
                        sum_calc_operations(&mut op_index, &mut var, &mut sum);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Subtract => {
                        op_index.index = 3;
                        sum_calc_operations(&mut op_index, &mut var, &mut sum);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Multiply => {
                        op_index.index = 4;
                        sum_calc_operations(&mut op_index, &mut var, &mut sum);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Divide => {
                        op_index.index = 5;
                        sum_calc_operations(&mut op_index, &mut var, &mut sum);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Sum => {
                        op_index.index = 6;
                        sum_calc_operations(&mut op_index, &mut var, &mut sum);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num0 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(0);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num1 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(1);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num2 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(2);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num3 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(3);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num4 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(4);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num5 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(5);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num6 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(6);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num7 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(7);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num8 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(8);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::Num9 => {
                        if op_index.index == 6 {
                            var.clear();
                            op_index.index = 0;
                        }
                        var.push(9);
                        click_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::NoneButtonBody => {
                        body_animation(&asset_server, &mut commands, *entity);
                    },
                    CalcButtons::NoneButtonScreen => {
                        screen_albedo.state = 0;
                        let color_count = MeshColor::VARIANT_COUNT;
                        if op_index.screen_color >= color_count {
                            op_index.screen_color = 0;
                        } else {
                            op_index.screen_color += 1;
                        }
                    },
                    CalcButtons::NoneButtonLightPanel => {
                        screen_albedo.state = 1;
                    },
                }
            } 
        }
    }
}

pub fn spawn_gltf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
    mut op_index: ResMut<OpIndex>,
) {
    let gltf = ass.load("calculator.glb#Scene0"); // Screen text is set in calculator.rs via CurrentMeshColor::process_entity_children

    // Scene
    commands.spawn(SceneBundle {
        scene: gltf,
        ..Default::default()
    })
    .insert(Interactable); 

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
    op_index.add_entity();

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    op_index.add_entity();
}

pub fn dim_while_clicked(
    mut op_index: ResMut<OpIndex>,
) {
    op_index.screen_color = 5;
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

pub fn body_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MeshAnimation)>,
    mut commands: Commands,
) {
    for (mut transform, mut animation) in query.iter_mut() {
        animation.progress += time.delta_seconds();

        let factor = (animation.progress / animation.duration).min(1.0);
        transform.scale = animation.initial_scale.lerp(animation.target_scale, factor);

        if factor >= 1.0 {
            // Remove the animation component once the animation is complete
            commands.entity(animation.target_entity).remove::<MeshAnimation>();
        }
    }
}

pub fn body_animation(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity).insert(MeshAnimation {
        progress: 0.0,
        duration: 0.125,
        initial_scale: Vec3::new(1.02, 1.02, 1.02),
        target_scale: Vec3::ONE,
        target_entity: entity,
    });
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/tap.mp3"),
        settings: PlaybackSettings::ONCE,
    });
}

pub fn button_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MeshAnimation)>,
    mut commands: Commands,
) {
    for (mut transform, mut animation) in query.iter_mut() {
        animation.progress += time.delta_seconds();

        let factor = (animation.progress / animation.duration).min(1.0);
        transform.scale = animation.initial_scale.lerp(animation.target_scale, factor);

        if factor >= 1.0 {
            // Remove the animation component once the animation is complete
            commands.entity(animation.target_entity).remove::<MeshAnimation>();
        }
    }
}

pub fn click_animation(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity).insert(MeshAnimation {
        progress: 0.0,
        duration: 0.18,
        initial_scale: Vec3::new(1.0, 0.88, 1.0),
        target_scale: Vec3::ONE,
        target_entity: entity,
    });
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/click.mp3"),
        settings: PlaybackSettings::ONCE,
    });
}

#[derive(Resource)]
pub struct CountdownCycle {
    pub timer: Timer,           // Set single timer for countdown
    pub loop_count: u32,        // Number of loops, currently tied to the varient_count to loop through all dynamically
    pub current_count: u32,     // Tracks where in the loop you are
    pub is_active: bool,        // Tracks if the loop is active
}

impl CountdownCycle {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.125, TimerMode::Once), // Set single timer for countdown
            loop_count: MeshColor::VARIANT_COUNT + 1, // +1 accounts for indexed logic
            current_count: 0,
            is_active: false,  // Initially inactive
        }
    }
}

impl Default for CountdownCycle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Asset, Component, TypePath)]
pub struct Interactable; 

#[derive(Component)]
pub struct Loaded;

#[derive(Component)] 
pub struct MeshAnimation {
    progress: f32,
    duration: f32,
    initial_scale: Vec3,
    target_scale: Vec3,
    target_entity: Entity,
}