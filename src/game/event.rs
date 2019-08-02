use super::block::{Block, BlockIndex, BlockIndexOffset};
use super::piece::Piece;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PutResult {
    pub block: Block,
    pub index: BlockIndex
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemoveResult {
    pub block: Block,
    pub index: BlockIndex
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveResult {
    pub block: Block,
    pub source: BlockIndex,
    pub destination: BlockIndex
}