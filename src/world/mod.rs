use bevy::prelude::*;

use crate::{materials::VoxelMaterial, world::chunk::Chunk};
pub(super) mod chunk;
mod debug;

pub(crate) use debug::ToggleChunkBoundaryOverlayEvent;

pub(super) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_plugins(debug::DebugPlugin);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut voxel_materials: ResMut<Assets<VoxelMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let mut chunk = Chunk::full();
    chunk.set_block(&[1, 2, 3], false);
    let cube_mesh = chunk.build_mesh();
    let cube_mesh_handle = meshes.add(cube_mesh);
    let cube_texture_handle: Handle<Image> = asset_server.load("textures/stone.png");
    let cube_material_handle = voxel_materials.add(VoxelMaterial {
        texture: Some(cube_texture_handle),
    });

    commands.spawn(MaterialMeshBundle {
        material: cube_material_handle,
        transform: Transform::from_xyz(0., 0., 0.),
        mesh: cube_mesh_handle,
        ..default()
    });

    commands.insert_resource(AmbientLight {
        brightness: 1.,
        ..default()
    });
}
