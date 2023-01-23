use flowmbed_dynsys::core::{Float, Bool};

pub trait OneShotAnalog {
  fn read(&mut self) -> anyhow::Result<Float>;
}

pub trait OneShotDigital {
  fn read(&mut self) -> anyhow::Result<Bool>;
}