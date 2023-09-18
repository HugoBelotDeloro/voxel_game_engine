//! Create a custom material to draw basic lines in 3D

use bevy::prelude::*;

mod material;
mod player_controller;
mod player;
mod test_scene;

use material::LineMaterial;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<LineMaterial>::default(),
            test_scene::TestScenePlugin,
            player::PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
                Press 'P' to toggle perspective for line gizmos\n\
                Hold 'Left' or 'Right' to change the line width",
            TextStyle {
                font_size: 20.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

