use super::StorageSize;
use super::{DefaultSystemStrorage, SystemStorageBuilder};

pub trait Block {
  const BLOCK_SIZE: StorageSize;
}

pub trait BlockBuilder<'a, T> {
  fn build<ST: DefaultSystemStrorage>(self, storage_builder: &mut SystemStorageBuilder<'a, ST>) -> T;
}