use flowmbed_dynsys::core::Float;

pub trait OneShotAnalog {
  fn read(&mut self) -> anyhow::Result<Float>;
}