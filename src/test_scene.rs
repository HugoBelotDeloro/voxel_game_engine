use crate::material::{LineList, LineMaterial};
use bevy::prelude::*;

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<LineMaterial>>,
    color: Color,
    transform: Transform,
) {
    // Spawn a list of lines with start and end points for each lines
    commands.spawn((
        MaterialMeshBundle {
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
            material: materials.add(LineMaterial { color }),
            ..default()
        },
        //PlayerController::default(),
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::GREEN,
        Transform::from_xyz(3., 0., 0.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::RED,
        Transform::from_xyz(0., 3., 0.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::BLUE,
        Transform::from_xyz(0., 0., 3.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::CYAN,
        Transform::from_xyz(-3., 0., 0.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::PINK,
        Transform::from_xyz(0., -3., 0.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::YELLOW,
        Transform::from_xyz(0., 0., -3.),
    );
}
