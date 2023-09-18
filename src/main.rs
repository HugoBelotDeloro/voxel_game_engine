//! Create a custom material to draw basic lines in 3D

use bevy::prelude::*;

mod material;
mod player_controller;
use player_controller::{player_controller, PlayerController};

use material::{LineList, LineMaterial};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_controller, move_camera))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    // Spawn a list of lines with start and end points for each lines
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineList {
            lines: vec![
                (Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)),
                (Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0)),
                (Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0)),
                (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
                (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
                (Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
                (Vec3::new(0.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
                (Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
                (Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0)),
                (Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
                (Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
                (Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
            ],
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(LineMaterial {
            color: Color::GREEN,
        }),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.5, 0.5, 5.0)
                .looking_at(Vec3::new(0.5, 0.5, 0.0), Vec3::Y),
            ..default()
        },
        PlayerController::default(),
    ));
}

fn move_camera(
    mut query: Query<(&mut Transform, &PlayerController), With<Camera>>,
    timer: Res<Time>,
) {
    for (mut transform, player_controller) in query.iter_mut() {
        transform.translation += Vec3::new(
            player_controller.horizontal_movement.x,
            player_controller.vertical_movement,
            player_controller.horizontal_movement.y,
        ) * timer.delta_seconds();

        let mouse_delta = player_controller.mouse_delta * timer.delta_seconds() * 0.1;

        transform.rotation = transform.rotation * Quat::from_rotation_x(mouse_delta.y);
        transform.rotation = Quat::from_rotation_y(mouse_delta.x) * transform.rotation;
    }
}
