use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

#[derive(Component)]
pub struct Interactable; // Marker component to identify interactable entities\

pub fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let gltf = ass.load("calculator.glb#Scene0");

    commands.spawn(SceneBundle {
        scene: gltf,
        ..Default::default()
    })
    .insert(Interactable) // Custom marker to identify this as interactable
    .insert(Name::new("CalculatorRoot")); // Optional: Naming root for easy reference

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}