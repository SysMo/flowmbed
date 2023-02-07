// use super

use super::system::{DynamicalSystem, SystemStateInfo};
use super::Float;

pub trait SystemRunner {
  fn init(&mut self, system: &mut dyn DynamicalSystem) -> anyhow::Result<()>;
  // fn step(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()>;
  fn run(&mut self, system: &mut dyn DynamicalSystem) -> anyhow::Result<()>;
}

pub struct FixedStepRunSettings {
  pub t_step: Float,
  pub t_print: Option<Float>,
  pub t_end: Option<Float>,
  pub speedup: Float
}

impl Default for FixedStepRunSettings {
  fn default() -> Self {
    FixedStepRunSettings {
      t_step: 0.01,
      t_print: None,
      t_end: None,
      speedup: 1.0,
    } 
  }
}

#[allow(dead_code)]
pub struct FixedStepRunner {
  settings: FixedStepRunSettings,
  t_last_print: Float,
}

impl FixedStepRunner {
  pub fn new(settings: FixedStepRunSettings) -> Self {
    FixedStepRunner {
      settings: settings, t_last_print: 0.0
    }
  }

  fn is_done(&self, ssi: &SystemStateInfo) -> bool {
    match self.settings.t_end {
      Some(t_end) => ssi.t > t_end,
      None => false
    }
  }
}

impl SystemRunner for FixedStepRunner {
  fn init(&mut self, system: &mut dyn DynamicalSystem) -> anyhow::Result<()> {
    system.init()
  }

  fn run(&mut self, system: &mut dyn DynamicalSystem) -> anyhow::Result<()> {
    let mut ssi = SystemStateInfo {t: 0.0};
    let sleep_interval = std::time::Duration::from_micros(
      (self.settings.t_step * 1e6 / self.settings.speedup).round() as u64
    );

    while !self.is_done(&ssi) {
      ssi.t += self.settings.t_step;
      system.step(&ssi)?;
      std::thread::sleep(sleep_interval);
    }
    Ok(())
  }
}