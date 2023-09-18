//! Create a custom material to draw basic lines in 3D

use std::f32::consts::{PI, TAU};

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

fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<LineMaterial>>,
    color: Color,
    transform: Transform,
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
        transform,
        material: materials.add(LineMaterial {
            color,
        }),
        ..default()
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    spawn_cube(&mut commands, &mut meshes, &mut materials, Color::GREEN, Transform::from_xyz(3., 0., 0.));
    spawn_cube(&mut commands, &mut meshes, &mut materials, Color::RED, Transform::from_xyz(0., 3., 0.));
    spawn_cube(&mut commands, &mut meshes, &mut materials, Color::BLUE, Transform::from_xyz(0., 0., 3.));
    spawn_cube(&mut commands, &mut meshes, &mut materials, Color::CYAN, Transform::from_xyz(-3., 0., 0.));
    spawn_cube(&mut commands, &mut meshes, &mut materials, Color::PINK, Transform::from_xyz(0., -3., 0.));
    spawn_cube(&mut commands, &mut meshes, &mut materials, Color::YELLOW, Transform::from_xyz(0., 0., -3.));

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
        transform.translation += Vec3::new(
            player_controller.horizontal_movement.x,
            player_controller.vertical_movement,
            player_controller.horizontal_movement.y,
        ) * timer.delta_seconds();

        let mouse_delta = player_controller.mouse_delta * timer.delta_seconds() * 0.2;

        let (mut pitch, mut yaw, _) = transform.rotation.to_euler(EulerRot::XYZ);

        pitch = (pitch + mouse_delta.y).clamp(-PI, PI);
        yaw += mouse_delta.x;
        if yaw.abs() > PI {
            yaw = yaw.rem_euclid(TAU);
        }

        transform.rotation = Quat::from_euler(EulerRot::XYZ, pitch, yaw, 0.);
    }
}
