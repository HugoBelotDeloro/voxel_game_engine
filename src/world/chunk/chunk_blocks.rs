use super::{BlockType, ChunkBlockCoords, CHUNK_SIZE, CHUNK_SIZE_USIZE};

const CHUNK_ARRAY_SIZE: usize = CHUNK_SIZE_USIZE.pow(3);

pub(super) struct ChunkBlocks {
    blocks: [BlockType; CHUNK_ARRAY_SIZE],
}

#[derive(Clone, Copy)]
pub(super) struct ChunkBlockIndex {
    n: usize,
}

pub(super) fn index_of(coords: &ChunkBlockCoords) -> ChunkBlockIndex {
    let [x, y, z] = *coords;
    assert!(x < CHUNK_SIZE);
    assert!(y < CHUNK_SIZE);
    assert!(z < CHUNK_SIZE);
    let (x, y, z) = (x as usize, y as usize, z as usize);
    ChunkBlockIndex {
        n: (x * CHUNK_SIZE_USIZE + y) * CHUNK_SIZE_USIZE + z,
    }
}

pub(super) fn chunk_coords_of(index: ChunkBlockIndex) -> ChunkBlockCoords {
    const CHUNK_SIZE_SQUARED: usize = CHUNK_SIZE_USIZE.pow(2);
    let x = index.n / CHUNK_SIZE_SQUARED;
    let y = (index.n % CHUNK_SIZE_SQUARED) / CHUNK_SIZE_USIZE;
    let z = index.n % CHUNK_SIZE_USIZE;

    [x as u8, y as u8, z as u8]
}

impl ChunkBlocks {
    pub(super) fn empty() -> Self {
        Self {
            blocks: [false; CHUNK_SIZE_USIZE.pow(3)],
        }
    }

    pub(super) fn full() -> Self {
        Self {
            blocks: [true; CHUNK_SIZE_USIZE.pow(3)],
        }
    }

    pub(super) fn get_block_with(&self, coords: &ChunkBlockCoords) -> BlockType {
        self.get_block(index_of(coords))
    }

    pub(super) fn get_block(&self, index: ChunkBlockIndex) -> BlockType {
        self.blocks[index.n]
    }

    pub(super) fn set_block_with(&mut self, coords: &ChunkBlockCoords, block_type: BlockType) {
        self.set_block(index_of(coords), block_type)
    }

    pub(super) fn set_block(&mut self, index: ChunkBlockIndex, block_type: BlockType) {
        self.blocks[index.n] = block_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords_to_index_conversions() {
        for n in 0..CHUNK_ARRAY_SIZE {
            let index = ChunkBlockIndex { n };
            assert_eq!(index_of(&chunk_coords_of(index)).n, n);
        }
    }
}
