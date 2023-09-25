mod line_material;
mod voxel_material;

use bevy::prelude::*;

pub(crate) use self::{line_material::LineMaterial, voxel_material::VoxelMaterial};

pub(super) struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<VoxelMaterial>::default(),
            MaterialPlugin::<LineMaterial>::default(),
        ));
    }
}
