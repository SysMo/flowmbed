use super::StorageSize;

pub trait Block {
  const size: StorageSize;
}