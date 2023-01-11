use flowmbed_core_blocks::sources;
use flowmbed_dynsys::core;
use flowmbed_dynsys::core::{DynamicalSystem, RequiresStorage};
use const_default::ConstDefault;

pub fn main() -> anyhow::Result<()> {
  const SIZE: core::StorageSize =
    core::StorageSize::DEFAULT
      .add(sources::SquareWaveSource::SIZE);
  let storage = core::HeapSystemStorage::new(SIZE);
  let mut storage_builder = core::SystemStorageBuilder::new(&storage);
  let mut s1: sources::SquareWaveSource = sources::SquareWaveSource::builder(&mut storage_builder).period(0.5).into();
  s1.init()?;
  println!("All ok!");
  Ok(())
}