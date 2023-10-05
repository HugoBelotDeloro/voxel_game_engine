use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use chunk_blocks::ChunkBlocks;
use chunk_coords_iterator::ChunkCoordsIterator;
use strum_macros::EnumIter;

mod chunk_blocks;
mod chunk_coords_iterator;

pub(super) const CHUNK_SIZE: u8 = 8;
pub(super) const CHUNK_SIZE_USIZE: usize = CHUNK_SIZE as usize;

#[derive(Component)]
pub(super) struct Chunk {
    blocks: ChunkBlocks,
}

type ChunkBlockCoords = [u8; 3];
pub(super) type BlockType = bool;

/// Used for the following code.
#[derive(EnumIter)]
enum Face {
    Front = 0,
    Back = 1,
    Right = 2,
    Left = 3,
    Top = 4,
    Bottom = 5,
}

const fn normals() -> [[f32; 3]; 6] {
    let mut arr = [[0.; 3]; 6];

    arr[Face::Front as usize] = [0., 0., 1.];
    arr[Face::Back as usize] = [0., 0., -1.];
    arr[Face::Right as usize] = [1., 0., 0.];
    arr[Face::Left as usize] = [-1., 0., 0.];
    arr[Face::Top as usize] = [0., 1., 0.];
    arr[Face::Bottom as usize] = [0., -1., 0.];

    arr
}

const fn uv_mappings() -> [[[f32; 2]; 4]; 6] {
    let mut arr = [[[0.; 2]; 4]; 6];

    arr[Face::Front as usize] = [[0., 1.], [0.5, 1.], [0.5, 0.5], [0.5, 1.]];
    arr[Face::Back as usize] = [[0.5, 0.], [0., 0.], [0., 0.5], [0.5, 0.5]];
    arr[Face::Right as usize] = [[0.5, 0.5], [0.5, 0.], [0., 0.], [0., 0.5]];
    arr[Face::Left as usize] = [[0.5, 0.5], [0.5, 0.], [0., 0.], [0., 0.5]];
    arr[Face::Top as usize] = [[1., 0.], [0.5, 0.], [0.5, 0.5], [1., 0.5]];
    arr[Face::Bottom as usize] = [[1., 0.], [0.5, 0.], [0.5, 0.5], [1., 0.5]];

    arr
}

const UV_MAPPINGS: [[[f32; 2]; 4]; 6] = uv_mappings();

const NORMALS: [[f32; 3]; 6] = normals();

impl Chunk {
    pub(super) fn full() -> Self {
        Self {
            blocks: ChunkBlocks::full(),
        }
    }

    pub(super) fn half_empty() -> Self {
        let mut chunk = Self {
            blocks: ChunkBlocks::empty(),
        };

        for [x, y, z] in Chunk::chunk_coords_iter() {
            if (x + y + z) % 2 == 0 {
                chunk.set_block(&[x, y, z], true);
            }
        }

        chunk
    }

    pub(super) fn get_block(&self, coords: &ChunkBlockCoords) -> BlockType {
        self.blocks.get_block_with(coords)
    }

    pub(super) fn set_block(&mut self, coords: &ChunkBlockCoords, block_type: BlockType) {
        self.blocks.set_block_with(coords, block_type)
    }

    pub(super) fn build_mesh(&self) -> Mesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        let mut current_index = 0u32;

        for [i, j, k] in Chunk::chunk_coords_iter() {
            let pos = [i, j, k];
            if self.get_block(&pos) {
                let min_x = (i) as f32;
                let max_x = (i + 1) as f32;
                let min_y = (j) as f32;
                let max_y = (j + 1) as f32;
                let min_z = (k) as f32;
                let max_z = (k + 1) as f32;

                let sides_to_render = [
                    k == CHUNK_SIZE - 1 || !self.has_block(&[i, j, k + 1]),
                    k == 0 || !self.has_block(&[i, j, k - 1]),
                    i == CHUNK_SIZE - 1 || !self.has_block(&[i + 1, j, k]),
                    i == 0 || !self.has_block(&[i - 1, j, k]),
                    j == CHUNK_SIZE - 1 || !self.has_block(&[i, j + 1, k]),
                    j == 0 || !self.has_block(&[i, j - 1, k]),
                ];

                let vertices_coordinates = [
                    [
                        [min_x, min_y, max_z],
                        [max_x, min_y, max_z],
                        [max_x, max_y, max_z],
                        [min_x, max_y, max_z],
                    ],
                    [
                        [min_x, max_y, min_z],
                        [max_x, max_y, min_z],
                        [max_x, min_y, min_z],
                        [min_x, min_y, min_z],
                    ],
                    [
                        [max_x, min_y, min_z],
                        [max_x, max_y, min_z],
                        [max_x, max_y, max_z],
                        [max_x, min_y, max_z],
                    ],
                    [
                        [min_x, min_y, max_z],
                        [min_x, max_y, max_z],
                        [min_x, max_y, min_z],
                        [min_x, min_y, min_z],
                    ],
                    [
                        [max_x, max_y, min_z],
                        [min_x, max_y, min_z],
                        [min_x, max_y, max_z],
                        [max_x, max_y, max_z],
                    ],
                    [
                        [max_x, min_y, max_z],
                        [min_x, min_y, max_z],
                        [min_x, min_y, min_z],
                        [max_x, min_y, min_z],
                    ],
                ];

                let cube_positions: Vec<[f32; 3]> = vertices_coordinates
                    .iter()
                    .zip(sides_to_render.iter())
                    .filter_map(|(p, side)| side.then_some(p))
                    .flat_map(|p| *p)
                    .collect();
                let cube_normals: Vec<_> = NORMALS
                    .iter()
                    .zip(sides_to_render.iter())
                    .filter_map(|(n, side)| side.then_some(n))
                    .flat_map(|n| [*n; 4])
                    .collect();
                let cube_uvs: Vec<_> = UV_MAPPINGS
                    .iter()
                    .zip(sides_to_render.iter())
                    .filter_map(|(uv, side)| side.then_some(uv))
                    .flat_map(|uv| *uv)
                    .collect();

                positions.extend(cube_positions);
                normals.extend(cube_normals);
                uvs.extend(cube_uvs);

                const LOCAL_INDICES: [[u32; 6]; 6] = [
                    [0, 1, 2, 2, 3, 0],
                    [4, 5, 6, 6, 7, 4],
                    [8, 9, 10, 10, 11, 8],
                    [12, 13, 14, 14, 15, 12],
                    [16, 17, 18, 18, 19, 16],
                    [20, 21, 22, 22, 23, 20],
                ];
                let count = sides_to_render.iter().filter(|b| **b).count();
                indices.extend(
                    LOCAL_INDICES
                        .iter()
                        .take(count)
                        .flat_map(|v| v.map(|i| i + current_index))
                        .collect::<Vec<u32>>(),
                );
                current_index += 4 * count as u32;
            }
        }

        let indices = Indices::U32(indices);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));

        mesh
    }

    fn chunk_coords_iter() -> ChunkCoordsIterator {
        ChunkCoordsIterator::new()
    }

    fn has_block(&self, coords: &ChunkBlockCoords) -> bool {
        self.blocks.get_block_with(coords)
    }
}
