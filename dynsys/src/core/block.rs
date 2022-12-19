use super::StorageSize;

pub trait Block {
  const BLOCK_SIZE: StorageSize;
}