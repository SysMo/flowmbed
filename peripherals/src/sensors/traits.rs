use flowmbed_dynsys::core::{Float, Bool};

pub trait OneShotAnalog {
  fn read(&mut self) -> anyhow::Result<Float>;
}

pub trait OneShotDigital {
  fn read(&mut self) -> anyhow::Result<Bool>;
}

pub trait AnalogReaderMultiChannel<const N: usize> {
  fn read_channel(&mut self, id: usize) -> anyhow::Result<Float>;
  fn read_all(&mut self) -> anyhow::Result<[Float; N]> {
    let mut values = [0.0; N];
    for i in 0..N {
      values[i] = self.read_channel(i)?;
    }
    Ok(values)
  }
}