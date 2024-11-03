use bevy::prelude::*;
use bevy::render::{
    camera::RenderTarget,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

use crate::{SumCurrent, SumVariable};

#[derive(Component)]
pub struct SumText;

pub fn update_sum_text(
    mut sum: ResMut<SumCurrent>,
    mut var: ResMut<SumVariable>,
    mut query: Query<&mut Text, With<SumText>>,
) {
    if sum.is_changed() {
        // Only run this if the `sum` resource has been changed.
        if *&sum.sum.to_string().len() > 8 {
            for mut text in &mut query {
                SumCurrent::zero(&mut sum);
                var.clear();
                text.sections[0].value = "S: Overload".to_owned();
            }
        } else {
            for mut text in &mut query {
                text.sections[0].value = "Sum: ".to_owned() + &sum.sum.to_string();
            }
        }
    }
}

#[derive(Component)]
pub struct VarText;

pub fn update_var_text(
    mut sum: ResMut<SumCurrent>,
    mut var: ResMut<SumVariable>,
    mut query: Query<&mut Text, With<VarText>>,
) {
    let mut res = if var.decimal_index > 0 {
        let mut num: String = "".to_string();
        let mut multiplier: String = ".".to_string();
        
        for i in 0..var.var.len() {
            num += &var.var[i].to_string();
        }
        
        for _ in 0..var.var.len() - var.decimal_index as usize {
            multiplier += "0";
        }
        
        let popped_multiplier = {
            let mut chars = multiplier.chars();
            chars.next_back();
            chars.as_str()
        };

        multiplier = popped_multiplier.to_string();
        multiplier += "1";

        let res_num: f64 = num.to_string().parse::<f64>().unwrap();
        let res_mul: f64 = multiplier.to_string().parse::<f64>().unwrap();
        let res_sum = res_num * res_mul;

        res_sum
    } else {
        let mut num: String = "".to_string();
        
        for i in 0..var.var.len() {
            num += &var.var[i].to_string();
        }
        
        let res_sum: f64 = if var.var.len() == 0 {
            0.0
        } else {
            num.to_string().parse::<f64>().unwrap()
        };

        res_sum
    };

    let mut display_limiter = res.to_string();
    while display_limiter.len() > 6 {
        display_limiter.pop();
    }

    if var.is_changed() {
        if res.to_string().len() > 6 {
            for mut text in &mut query {
                SumCurrent::zero(&mut sum);
                var.clear();
                text.sections[0].value = "I: Overload".to_owned();
            }
        } else {
            // Only run this if the recompiled `Var` resource has been changed.
            for mut text in &mut query {
                text.sections[0].value = "Input: ".to_owned() + &display_limiter;
            }
        }
    }
}

pub fn setup_calc_interface_projection(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    sum: Res<SumCurrent>,
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

    let font = asset_server.load("fonts/MatrixtypeDisplay-KVELZ.ttf"); 
    
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
                "Input: 0",
                TextStyle {
                    font: font.clone(),
                    font_size: 110.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                    ..default()
                },
            ))
            .insert(VarText); // Insert a marker component to easily query this later
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Sum: ".to_owned() + &sum.sum.to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 110.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                    ..default()
                },
            ))
            .insert(SumText); // Insert a marker component to easily query this later
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