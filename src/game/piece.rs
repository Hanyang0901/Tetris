use super::block::{Block, BlockGridSize, BlockIndex};
use std::iter;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Piece {
    size: BlockGridSize,
    block: Block,
    indices: Vec<BlockIndex>,
}




pub fn standards() -> Vec<Piece> {
    [
        (4, [(0, 2), (1, 2), (2, 2), (3, 2)]), // I
        (2, [(0, 0), (1, 0), (0, 1), (1, 1)]), // O
        (3, [(1, 2), (2, 2), (0, 1), (1, 1)]), // S
        (3, [(0, 2), (1, 2), (1, 1), (2, 1)]), // Z
        (3, [(0, 2), (0, 1), (1, 1), (2, 1)]), // J
        (3, [(2, 2), (0, 1), (1, 1), (2, 1)]), // L
        (3, [(1, 2), (0, 1), (1, 1), (2, 1)]), // T
    ]
        .iter()
        .enumerate()
        .map(|(number, &(size, indices))| Piece {
            size: BlockGridSize::new(size, size),
            block: Block::new(number as u32),
            indices: indices
                .iter()
                .map(|&(x, y)| BlockIndex::new(x, y))
                .collect::<Vec<_>>(),
        })
        .collect()
}
