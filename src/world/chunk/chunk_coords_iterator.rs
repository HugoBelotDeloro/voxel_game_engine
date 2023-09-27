use super::{ChunkBlockCoords, CHUNK_SIZE};

pub(super) struct ChunkCoordsIterator {
    coords: ChunkBlockCoords,
}

impl ChunkCoordsIterator {
    pub(super) fn new() -> Self {
        Self { coords: [0, 0, 0] }
    }
}

impl Iterator for ChunkCoordsIterator {
    type Item = ChunkBlockCoords;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.coords;
        let [mut x, mut y, mut z] = self.coords;
        if z + 1 == CHUNK_SIZE {
            if y + 1 == CHUNK_SIZE {
                if x + 1 == CHUNK_SIZE {
                    return None;
                } else {
                    x += 1;
                }
                y = 0;
            } else {
                y += 1;
            }
            z = 0;
        } else {
            z += 1;
        }

        self.coords = [x, y, z];
        Some(item)
    }
}
