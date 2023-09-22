use crate::material::CustomMaterial;
use bevy::prelude::*;

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_plugins(MaterialPlugin::<CustomMaterial>::default());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
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
    let cube_material_handle = custom_materials.add(CustomMaterial {
        color_texture: Some(cube_texture_handle),
    });

    commands.spawn(MaterialMeshBundle {
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
