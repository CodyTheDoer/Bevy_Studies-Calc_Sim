use bevy::{color::palettes::css::*, sprite::Anchor, text::{BreakLineOn, Text2dBounds}};  
use bevy::prelude::*;

// UI with direct spawn
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let my_2d_camera_bundle = Camera2dBundle {
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