pub struct SystemStateInfo {
  pub t: f64,
  // pub event: bool
}


pub trait DynamicalSystem {
  fn connect(&mut self) -> anyhow::Result<()> { Ok(()) }
  fn init(&mut self) -> anyhow::Result<()>;
  fn step(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()>;
}

