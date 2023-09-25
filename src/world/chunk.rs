use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

pub(super) const CHUNK_SIZE: usize = 8;

#[derive(Component)]
pub(super) struct Chunk {
    blocks: Vec<bool>,
}

impl Chunk {
    pub(super) fn half_empty() -> Self {
        let mut blocks = vec![false; CHUNK_SIZE.pow(3)];

        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                for k in 0..CHUNK_SIZE {
                    if (i + j + k) % 2 == 0 {
                        blocks[(i * CHUNK_SIZE + j) * CHUNK_SIZE + k] = true;
                    }
                }
            }
        }
        Chunk { blocks }
    }

    pub(super) fn get_block(&self, [x, y, z]: &[usize; 3]) -> bool {
        self.blocks[(x * CHUNK_SIZE + y) * CHUNK_SIZE + z]
    }

    pub(super) fn set_block(&mut self, [x, y, z]: &[usize; 3], value: bool) {
        self.blocks[(x * CHUNK_SIZE + y) * CHUNK_SIZE + z] = value;
    }

    pub(super) fn build_mesh(&self) -> Mesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        let mut current_index = 0u32;

        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                for k in 0..CHUNK_SIZE {
                    let pos = [i, j, k];
                    if self.get_block(&pos) {
                        let min_x = (i) as f32;
                        let max_x = (i + 1) as f32;
                        let min_y = (j) as f32;
                        let max_y = (j + 1) as f32;
                        let min_z = (k) as f32;
                        let max_z = (k + 1) as f32;
                        let vertices = &[
                            // (coordinates, normals, uv mapping)
                            // Front
                            ([min_x, min_y, max_z], [0., 0., 1.0], [0., 1.]),
                            ([max_x, min_y, max_z], [0., 0., 1.0], [0.5, 1.]),
                            ([max_x, max_y, max_z], [0., 0., 1.0], [0.5, 0.5]),
                            ([min_x, max_y, max_z], [0., 0., 1.0], [0.5, 1.]),
                            // Back
                            ([min_x, max_y, min_z], [0., 0., -1.0], [0.5, 0.]),
                            ([max_x, max_y, min_z], [0., 0., -1.0], [0., 0.]),
                            ([max_x, min_y, min_z], [0., 0., -1.0], [0., 0.5]),
                            ([min_x, min_y, min_z], [0., 0., -1.0], [0.5, 0.5]),
                            // Right
                            ([max_x, min_y, min_z], [1.0, 0., 0.], [0.5, 0.5]),
                            ([max_x, max_y, min_z], [1.0, 0., 0.], [0.5, 0.]),
                            ([max_x, max_y, max_z], [1.0, 0., 0.], [0., 0.]),
                            ([max_x, min_y, max_z], [1.0, 0., 0.], [0., 0.5]),
                            // Left
                            ([min_x, min_y, max_z], [-1.0, 0., 0.], [0.5, 0.5]),
                            ([min_x, max_y, max_z], [-1.0, 0., 0.], [0.5, 0.]),
                            ([min_x, max_y, min_z], [-1.0, 0., 0.], [0., 0.]),
                            ([min_x, min_y, min_z], [-1.0, 0., 0.], [0., 0.5]),
                            // Top
                            ([max_x, max_y, min_z], [0., 1.0, 0.], [1., 0.]),
                            ([min_x, max_y, min_z], [0., 1.0, 0.], [0.5, 0.]),
                            ([min_x, max_y, max_z], [0., 1.0, 0.], [0.5, 0.5]),
                            ([max_x, max_y, max_z], [0., 1.0, 0.], [1., 0.5]),
                            // Bottom
                            ([max_x, min_y, max_z], [0., -1.0, 0.], [1., 0.]),
                            ([min_x, min_y, max_z], [0., -1.0, 0.], [0.5, 0.]),
                            ([min_x, min_y, min_z], [0., -1.0, 0.], [0.5, 0.5]),
                            ([max_x, min_y, min_z], [0., -1.0, 0.], [1., 0.5]),
                        ];

                        let cube_positions: Vec<[f32; 3]> =
                            vertices.iter().map(|(p, _, _)| *p).collect();
                        let cube_normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
                        let cube_uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

                        positions.extend(cube_positions);
                        normals.extend(cube_normals);
                        uvs.extend(cube_uvs);
                        const LOCAL_INDICES: [u32; 36] = [
                            0, 1, 2, 2, 3, 0, // front
                            4, 5, 6, 6, 7, 4, // back
                            8, 9, 10, 10, 11, 8, // right
                            12, 13, 14, 14, 15, 12, // left
                            16, 17, 18, 18, 19, 16, // top
                            20, 21, 22, 22, 23, 20, // bottom
                        ];
                        indices.extend(
                            LOCAL_INDICES
                                .iter()
                                .map(|i| i + current_index)
                                .collect::<Vec<u32>>(),
                        );
                        current_index += 24;
                    }
                }
            }
        }
        // let top_left = [0., 0.];
        // let top_center = [0.5, 0.];
        // let top_right = [1., 0.];
        // let middle_left = [0., 0.5];
        // let middle_center = [0.5, 0.5];
        // let middle_right = [1., 0.5];
        // let bottom_left = [0., 1.];
        // let bottom_center = [0.5, 1.];
        // let bottom_right = [1., 1.];

        let indices = Indices::U32(indices);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));

        mesh
    }
}
