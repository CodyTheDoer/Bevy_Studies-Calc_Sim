use bevy::prelude::*;
use bevy::render::{
    camera::RenderTarget,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

pub fn setup_calc_interface_projection(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 1024,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // Light
    commands.spawn(DirectionalLightBundle::default());

    let texture_camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // Cover the whole image
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                ..default()
            },
            TargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "This is a calculator...",
                TextStyle {
                    font_size: 80.0,
                    color: Color::BLACK,
                    ..default()
                },
            ));
        });

    // This material has the texture that has been rendered.
    let calc_ui_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // Store the handle in a resource for future use
    commands.insert_resource(CalcUIMaterialHandle {
        material_handle: calc_ui_handle,
        image_handle,
    });
}

#[derive(Resource)]
pub struct CalcUIMaterialHandle {
    pub material_handle: Handle<StandardMaterial>,
    pub image_handle: Handle<Image>,  // Add this to store the image handle
}