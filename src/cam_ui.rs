use bevy::{color::palettes::css::*, sprite::Anchor, text::{BreakLineOn, Text2dBounds}};  
use bevy::prelude::*;

use crate::{OpIndex, SumCurrent};

use crate::cam_calc_screen::{SumText, VarText};

#[derive(Component)]
pub struct CameraUi;

// UI with direct spawn
pub fn setup_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    sum: Res<SumCurrent>,
    // op_index: &mut ResMut<OpIndex>,
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
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            camera: Camera {
                order: -1, // Render on top of the 3D scene
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
        parent
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.5), // Keep it on the right side of the window
                    bottom: Val::Percent(2.0), // General area at the bottom, but will contain both texts
                    flex_direction: FlexDirection::Column, // Stack elements vertically
                    align_items: AlignItems::FlexEnd, // Align text to the right within the container
                    padding: UiRect::all(Val::Px(5.0)), // Add padding to the container to avoid tight spacing
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {    
                // Input Text
                parent
                    .spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Input: 0",
                                smaller_text_style.clone(),
                            )],
                            ..default()
                        },
                        style: Style {
                            margin: UiRect {
                                top: Val::Px(5.0), // Add space between Sum and Input texts
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .insert(VarText); // Insert a marker component to easily query this later

                    
                // Sum Text
                parent
                    .spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Sum: ".to_owned() + &sum.sum.to_string(),
                                smaller_text_style.clone(),
                            )],
                            ..default()
                        },
                        ..default()
                    })
                    .insert(SumText); // Insert a marker component to easily query this later
            });
    });
}