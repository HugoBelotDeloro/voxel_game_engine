//! Create a custom material to draw basic lines in 3D

use bevy::prelude::*;

mod material;
mod player_controller;
mod test_scene;
use player_controller::{player_controller, PlayerController};

use material::LineMaterial;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<LineMaterial>::default(),
            test_scene::TestScenePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_controller, move_camera))
        .run();
}

fn setup(
    mut commands: Commands,
) {
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

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.5, 0.5, 0.5)
                .looking_at(Vec3::new(0.5, 0.5, -1.0), Vec3::Y),
            ..default()
        },
        PlayerController::default(),
    ));
}

fn move_camera(mut query: Query<(&mut Transform, &PlayerController)>, timer: Res<Time>) {
    for (mut transform, player_controller) in query.iter_mut() {
        let direction = transform.back() * player_controller.horizontal_movement.y
            + transform.right() * player_controller.horizontal_movement.x
            + transform.up() * player_controller.vertical_movement;
        transform.translation += direction * timer.delta_seconds();

        let mouse_delta = player_controller.mouse_delta * timer.delta_seconds() * 0.2;

        //transform.rotation = Quat::from_euler(EulerRot::XYZ, pitch, yaw, PI);
        //transform.rotate_x(mouse_delta.y);
        transform.rotate_y(mouse_delta.x);
    }
}
