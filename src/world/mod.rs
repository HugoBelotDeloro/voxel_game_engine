use bevy::prelude::*;

use crate::{
    materials::{line_material::LineMaterial, voxel_material::VoxelMaterial},
    world::chunk::{Chunk, CHUNK_SIZE},
};
pub mod chunk;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_plugins(MaterialPlugin::<VoxelMaterial>::default())
            .add_plugins(MaterialPlugin::<LineMaterial>::default());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut voxel_materials: ResMut<Assets<VoxelMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let chunk = Chunk::half_empty();
    let cube_mesh = chunk.build_mesh();
    let cube_mesh_handle = meshes.add(cube_mesh);
    let cube_texture_handle: Handle<Image> = asset_server.load("textures/stone.png");
    let cube_material_handle = voxel_materials.add(VoxelMaterial {
        color_texture: Some(cube_texture_handle),
    });

    commands.spawn(MaterialMeshBundle {
        material: cube_material_handle,
        transform: Transform::from_xyz(0., 0., 0.),
        mesh: cube_mesh_handle,
        ..default()
    });

    let line_material = line_materials.add(LineMaterial {
        color: Color::BLACK,
    });
    const OFFSET: f32 = CHUNK_SIZE as f32 / 2.;
    commands.spawn(MaterialMeshBundle {
        material: line_material,
        transform: Transform::from_xyz(OFFSET, OFFSET, OFFSET),
        mesh: meshes.add(Mesh::from(shape::Cube::new(CHUNK_SIZE as f32))),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        brightness: 1.,
        ..default()
    });
}
