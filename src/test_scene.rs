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
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        Color::GREEN,
        Transform::from_xyz(3., 0., 0.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        Color::RED,
        Transform::from_xyz(0., 3., 0.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        Color::BLUE,
        Transform::from_xyz(0., 0., 3.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        Color::CYAN,
        Transform::from_xyz(-3., 0., 0.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        Color::PINK,
        Transform::from_xyz(0., -3., 0.),
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut line_materials,
        Color::YELLOW,
        Transform::from_xyz(0., 0., -3.),
    );

    let mut cube_mesh = Mesh::from(shape::Cube::default());
    #[rustfmt::skip]
    cube_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![
        // Front
        [0., 1.], [0.5, 1.], [0.5, 0.5], [0.5, 1.],
        // Back
        [0.5, 0.], [0., 0.], [0., 0.5], [0.5, 0.5],
        // Right
        [0.5, 0.5], [0.5, 0.], [0., 0.], [0., 0.5],
        // Left
        [0.5, 0.5], [0.5, 0.], [0., 0.], [0., 0.5],
        // Top
        [1., 0.], [0.5, 0.], [0.5, 0.5], [1., 0.5],
        // Bottom
        [1., 0.], [0.5, 0.], [0.5, 0.5], [1., 0.5],
    ]);
    let cube_mesh_handle = meshes.add(cube_mesh);
    let cube_texture_handle: Handle<Image> = asset_server.load("textures/stone.png");
    let cube_material_handle = standard_materials.add(StandardMaterial {
        base_color_texture: Some(cube_texture_handle),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: cube_mesh_handle,
        material: cube_material_handle,
        ..default()
    });

    commands.insert_resource(AmbientLight {
        brightness: 1.,
        ..default()
    });

    // commands.insert_resource(Msaa::Off);
}
