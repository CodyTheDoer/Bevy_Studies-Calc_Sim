use bevy::{color::palettes::css::*, sprite::Anchor, text::{BreakLineOn, Text2dBounds}};  
use bevy::prelude::*;

use crate::{SumCurrent, SumVariable};

#[derive(Component)]
pub struct CameraUi;

#[derive(Component)]
pub struct SumText;

pub fn update_sum_text(
    sum: Res<SumCurrent>,
    mut query: Query<&mut Text, With<SumText>>,
) {
    if sum.is_changed() {
        // Only run this if the `sum` resource has been changed.
        for mut text in &mut query {
            text.sections[0].value = "Sum: ".to_owned() + &sum.sum.to_string();
        }
    }
}

#[derive(Component)]
pub struct VarText;

pub fn update_var_text(
    var: Res<SumVariable>,
    mut query: Query<&mut Text, With<VarText>>,
) {
    let mut res = if var.decimal_index > 0 {
        let mut num: String = "".to_string();
        let mut multiplier: String = ".".to_string();
        
        for i in 0..var.var.len() {
            num += &var.var[i].to_string();
        }
        
        for i in 0..var.var.len() - var.decimal_index as usize {
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

    if var.is_changed() {
        // Only run this if the recompiled `Var` resource has been changed.
        for mut text in &mut query {
            text.sections[0].value = "Input: ".to_owned() + &res.to_string();
        }
    }
}

// UI with direct spawn
pub fn setup_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    sum: Res<SumCurrent>,
) {
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
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(1000.0, 1000.0, 0.0),
            camera: Camera {
                order: 1, // Render on top of the 3D scene
                ..default()
                },
            ..default()
        },
        CameraUi,
    ));

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
                    text_style.clone(),
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
                    smaller_text_style.clone(),
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                // Manually set the position of the text to the bottom left
                left: Val::Percent(1.5),
                bottom: Val::Percent(2.0), 
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Sum: ".to_owned() + &sum.sum.to_string(),
                    smaller_text_style.clone(),
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.5),
                bottom: Val::Percent(2.0),
                ..default()
            },
            ..default()
        })
        .insert(SumText); // Insert a marker component to easily query this later
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Input: 0",
                    smaller_text_style.clone(),
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.5),
                bottom: Val::Percent(5.0),
                ..default()
            },
            ..default()
        })
        .insert(VarText); // Insert a marker component to easily query this later
    });
}