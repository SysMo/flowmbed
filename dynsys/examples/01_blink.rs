extern crate flowmbed_dynsys;

use flowmbed_dynsys::cfg_device;
use flowmbed_dynsys::core::{
  Block,
  StorageSize, DefaultSystemStrorage, HeapSystemStorage, SystemStorageBuilder, 
  SystemStateInfo, DynamicalSystem,  
  FixedStepRunner, FixedStepRunSettings, SystemRunner
};
use flowmbed_dynsys::block_library::{
  hal::{OutputPin, esp32_hal},
  sources::SquareSource,
  hardware_sinks::{DigitalOutput},
  discrete::CountingTrigger,
};

use esp32_hal::peripherals::Peripherals;
use esp32_hal::gpio;
use esp32_hal::gpio::{PinDriver, Output};

struct LedSystemPeripherals<'a> {
  led1: PinDriver<'a, gpio::Gpio15, Output>,
  led2: PinDriver<'a, gpio::Gpio2, Output>,
  led3: PinDriver<'a, gpio::Gpio4, Output>,
  led4: PinDriver<'a, gpio::Gpio16, Output>,
  led5: PinDriver<'a, gpio::Gpio17, Output>,
  led6: PinDriver<'a, gpio::Gpio5, Output>,
}

impl<'a> LedSystemPeripherals<'a> {
  pub fn new() -> LedSystemPeripherals<'a> {
    let peripherals = Peripherals::take().unwrap();
    LedSystemPeripherals {
      led1: PinDriver::output(peripherals.pins.gpio15).unwrap(),
      led2: PinDriver::output(peripherals.pins.gpio2).unwrap(), 
      led3: PinDriver::output(peripherals.pins.gpio4).unwrap(),
      led4: PinDriver::output(peripherals.pins.gpio16).unwrap(), 
      led5: PinDriver::output(peripherals.pins.gpio17).unwrap(),
      led6: PinDriver::output(peripherals.pins.gpio5).unwrap(), 
    }    
  }
}

struct LedSystemBlocks<'a> {
  source: SquareSource<'a>,
  trigger1: CountingTrigger<'a>,
  trigger2: CountingTrigger<'a>,
  trigger3: CountingTrigger<'a>,
  trigger4: CountingTrigger<'a>,
  trigger5: CountingTrigger<'a>,
  trigger6: CountingTrigger<'a>,
  led1: DigitalOutput<'a>,
  led2: DigitalOutput<'a>,
  led3: DigitalOutput<'a>,
  led4: DigitalOutput<'a>,
  led5: DigitalOutput<'a>,
  led6: DigitalOutput<'a>,
}

impl<'a> LedSystemBlocks<'a> {
  pub fn new<ST: DefaultSystemStrorage>(
    storage: &'a ST, peripherals: &'a mut LedSystemPeripherals
  ) -> LedSystemBlocks<'a> {
    let mut builder = SystemStorageBuilder::new(storage);
    let add = 1;
    // // Consequtive up, simultaneous down
    // let ups = [1, 2, 3, 4, 5, 6];
    // let downs = [6, 5, 4, 3, 2, 1];
    // let offset = [0, 0, 0, 0, 0, 0];

    // // Consequtive up, consequtive down, same direction
    // let ups = [7, 7, 7, 7, 7, 7];
    // let downs = [7, 7, 7, 7, 7, 7];
    // let offset = [5, 4, 3, 2, 1, 0];

    // Consequtive up, consequtive down, opposite direction
    let ups = [1, 3, 5, 7, 9, 11];
    let downs = [13, 11, 9, 7, 5, 3];
    let offset = [0, 1, 2, 3, 4, 5];

    let mut blocks = LedSystemBlocks {
      source: SquareSource::builder(&mut builder)
        .period(0.1).into(),
      trigger1: CountingTrigger::builder(&mut builder)
        .pulses_up(ups[0]).pulses_down(downs[0]).initial_count(offset[0]).into(),
      trigger2: CountingTrigger::builder(&mut builder)
        .pulses_up(ups[1]).pulses_down(downs[1]).initial_count(offset[1]).into(),
      trigger3: CountingTrigger::builder(&mut builder)
        .pulses_up(ups[2]).pulses_down(downs[2]).initial_count(offset[2]).into(),
      trigger4: CountingTrigger::builder(&mut builder)
        .pulses_up(ups[3]).pulses_down(downs[3]).initial_count(offset[3]).into(),
      trigger5: CountingTrigger::builder(&mut builder)
        .pulses_up(ups[4]).pulses_down(downs[4]).initial_count(offset[4]).into(),
      trigger6: CountingTrigger::builder(&mut builder)
        .pulses_up(ups[5]).pulses_down(downs[5]).initial_count(offset[5]).into(),
      
      led1: DigitalOutput::new(&mut builder, &mut peripherals.led1),
      led2: DigitalOutput::new(&mut builder, &mut peripherals.led2),
      led3: DigitalOutput::new(&mut builder, &mut peripherals.led3),
      led4: DigitalOutput::new(&mut builder, &mut peripherals.led4),
      led5: DigitalOutput::new(&mut builder, &mut peripherals.led5),
      led6: DigitalOutput::new(&mut builder, &mut peripherals.led6),
    };

    blocks.trigger1.input.connect(&blocks.source.output).unwrap();
    blocks.led1.input.connect(&blocks.trigger1.output).unwrap();

    blocks.trigger2.input.connect(&blocks.source.output).unwrap();
    blocks.led2.input.connect(&blocks.trigger2.output).unwrap();

    blocks.trigger3.input.connect(&blocks.source.output).unwrap();
    blocks.led3.input.connect(&blocks.trigger3.output).unwrap();

    blocks.trigger4.input.connect(&blocks.source.output).unwrap();
    blocks.led4.input.connect(&blocks.trigger4.output).unwrap();

    blocks.trigger5.input.connect(&blocks.source.output).unwrap();
    blocks.led5.input.connect(&blocks.trigger5.output).unwrap();

    blocks.trigger6.input.connect(&blocks.source.output).unwrap();
    blocks.led6.input.connect(&blocks.trigger6.output).unwrap();

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
    blocks.trigger1.init()?;
    blocks.trigger2.init()?;
    blocks.trigger3.init()?;
    blocks.trigger4.init()?;
    blocks.trigger5.init()?;
    blocks.trigger6.init()?;
    blocks.led1.init()?;
    blocks.led2.init()?;
    blocks.led3.init()?;
    blocks.led4.init()?;
    blocks.led5.init()?;
    blocks.led6.init()?;
    Ok(())
  }

  fn step(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    let blocks = &mut self.blocks;

    blocks.source.compute(ssi)?;
    blocks.trigger1.compute(ssi)?;
    blocks.trigger2.compute(ssi)?;
    blocks.trigger3.compute(ssi)?;
    blocks.trigger4.compute(ssi)?;
    blocks.trigger5.compute(ssi)?;
    blocks.trigger6.compute(ssi)?;
    blocks.led1.compute(ssi)?;
    blocks.led2.compute(ssi)?;
    blocks.led3.compute(ssi)?;
    blocks.led4.compute(ssi)?;
    blocks.led5.compute(ssi)?;
    blocks.led6.compute(ssi)?;
    Ok(())
  }
}

fn main() -> anyhow::Result<()> {
  
  cfg_device::config_logger();

  const SYSTEM_SIZE : StorageSize = 
    SquareSource::BLOCK_SIZE
    .add(CountingTrigger::BLOCK_SIZE)
    .add(CountingTrigger::BLOCK_SIZE)
    .add(CountingTrigger::BLOCK_SIZE)
    .add(CountingTrigger::BLOCK_SIZE)
    .add(CountingTrigger::BLOCK_SIZE)
    .add(CountingTrigger::BLOCK_SIZE)
    .add(DigitalOutput::BLOCK_SIZE)
    .add(DigitalOutput::BLOCK_SIZE)
    .add(DigitalOutput::BLOCK_SIZE)
    .add(DigitalOutput::BLOCK_SIZE)
    .add(DigitalOutput::BLOCK_SIZE)
    .add(DigitalOutput::BLOCK_SIZE)
    ;
  let storage = HeapSystemStorage::new(SYSTEM_SIZE);
  unsafe {
    flowmbed_dynsys::util::debug::STORAGE = Some(&storage as *const HeapSystemStorage);
  }
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
