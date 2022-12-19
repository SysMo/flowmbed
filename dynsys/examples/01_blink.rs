extern crate flowmbed_dynsys;

use flowmbed_dynsys::cfg_device;
use flowmbed_dynsys::core::{
  Block,
  StorageSize, DefaultSystemStrorage, HeapSystemStorage, SystemStorageBuilder, 
  SystemStateInfo, DynamicalSystem,  
  FixedStepRunner, FixedStepRunSettings, SystemRunner
};
use flowmbed_dynsys::block_library::{
  hal::{IOutputPin, esp32_hal},
  sources::SquareSource,
  hardware_sinks::{DigitalOutput},
  discrete::SimpleDelay,
};

use esp32_hal::peripherals::Peripherals;
use esp32_hal::gpio::{PinDriver, Output, Gpio2, Gpio4};

struct LedSystemPeripherals<'a> {
  led1: PinDriver<'a, Gpio2, Output>,
  led2: PinDriver<'a, Gpio4, Output>,
}

impl<'a> LedSystemPeripherals<'a> {
  pub fn new() -> LedSystemPeripherals<'a> {
    let peripherals = Peripherals::take().unwrap();
    let led1 = PinDriver::output(peripherals.pins.gpio2).unwrap();
    let led2 = PinDriver::output(peripherals.pins.gpio4).unwrap();
    LedSystemPeripherals {
      led1,
      led2, 
    }    
  }
}

struct LedSystemBlocks<'a> {
  source: SquareSource<'a>,
  delay: SimpleDelay<'a>,
  led1: DigitalOutput<'a>,
  led2: DigitalOutput<'a>
}

impl<'a> LedSystemBlocks<'a> {
  pub fn new<ST: DefaultSystemStrorage>(
    storage: &'a ST, peripherals: &'a mut LedSystemPeripherals
  ) -> LedSystemBlocks<'a> {
    let mut builder = SystemStorageBuilder::new(storage);

    let mut blocks = LedSystemBlocks {
      source: SquareSource::new(&mut builder),
      delay: SimpleDelay::new(&mut builder),
      led1: DigitalOutput::new(&mut builder, &mut peripherals.led1),
      led2: DigitalOutput::new(&mut builder, &mut peripherals.led2),
    };

    storage.set_parameter::<f64>(blocks.source.duty.id, 0.2);
    storage.set_parameter::<f64>(blocks.delay.delay.id, 0.15);

    blocks.led1.input.connect(&blocks.source.output).unwrap();
    blocks.delay.input.connect(&blocks.source.output).unwrap();
    blocks.led2.input.connect(&blocks.delay.output).unwrap();

    blocks
  }
}


struct LedSystem<'a> {
  blocks: LedSystemBlocks<'a>,
}

impl<'a> LedSystem<'a> {
  pub fn new<ST: DefaultSystemStrorage>(
    storage: &'a ST, peripherals: &'a mut LedSystemPeripherals
  ) -> LedSystem<'a> {
    LedSystem {
      blocks: LedSystemBlocks::new(storage, peripherals)
    }

  }
}

impl<'a> DynamicalSystem for LedSystem<'a> {
  fn init(&mut self) -> anyhow::Result<()> {
    let blocks = &mut self.blocks;

    blocks.source.init()?;
    blocks.led1.init()?;
    blocks.delay.init()?;
    blocks.led2.init()?;
    Ok(())
  }

  fn step(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    let blocks = &mut self.blocks;

    blocks.source.compute(ssi)?;
    blocks.led1.compute(ssi)?;
    blocks.delay.compute(ssi)?;
    blocks.led2.compute(ssi)?;
    Ok(())
  }
}

fn main() -> anyhow::Result<()> {
  
  cfg_device::config_logger();

  const SYSTEM_SIZE : StorageSize = 
    SquareSource::BLOCK_SIZE
    .add(DigitalOutput::BLOCK_SIZE)
    .add(SimpleDelay::BLOCK_SIZE)
    .add(DigitalOutput::BLOCK_SIZE)
    ;
  let storage = HeapSystemStorage::new(SYSTEM_SIZE);
  let mut peripherals = LedSystemPeripherals::new();
  let mut system = LedSystem::new(&storage, &mut peripherals);
  let run_settings = FixedStepRunSettings {
    t_step: 0.01, speedup: 1.0, t_end: None,
    ..Default::default()
  };
  let mut runner = FixedStepRunner::new(&mut system, run_settings);

  runner.init()?;
  runner.run()?;

  Ok(())
}
