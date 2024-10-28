use bevy::prelude::*;

pub fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("calculator.glb#Scene0");

    commands.spawn(SceneBundle {
        scene: my_gltf,
        ..Default::default()
    });

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}