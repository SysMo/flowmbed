// use super::{DefaultSystemStrorage, SystemStorageBuilder};
use super::Float;

pub struct SystemStateInfo {
  pub t: Float,
  // pub event: bool
}

pub trait DynamicalSystem<'a> {
  fn connect(&mut self) -> anyhow::Result<()> { Ok(()) }
  fn init(&mut self) -> anyhow::Result<()>;
  fn step(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()>;
}

