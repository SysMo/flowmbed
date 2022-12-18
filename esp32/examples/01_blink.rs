extern crate flowmbed_shared;

use flowmbed_shared::dynsys::system::{SystemStateInfo};
use flowmbed_shared::dynsys::{StorageSize, DefaultSystemStrorage, SystemStorageBuilder};
use flowmbed_shared::dynsys::HeapSystemStorage;
use flowmbed_shared::dynsys::Block;
use flowmbed_shared::dynsys::block_library::{
  sources::SquareSource,
  hardware_sinks::{DigitalOutput},
  discrete::SimpleDelay,
};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::{PinDriver, Output, Gpio2, Gpio4};

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

  pub fn init(&mut self) -> anyhow::Result<()> {
    let blocks = &mut self.blocks;

    blocks.source.init()?;
    blocks.led1.init()?;
    blocks.delay.init()?;
    blocks.led2.init()?;
    Ok(())
  }

  pub fn step(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    let blocks = &mut self.blocks;

    blocks.source.compute(ssi)?;
    blocks.led1.compute(ssi)?;
    blocks.delay.compute(ssi)?;
    blocks.led2.compute(ssi)?;
    Ok(())
  }

}

fn main() -> anyhow::Result<()> {
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();


  const SYSTEM_SIZE : StorageSize = 
    SquareSource::BLOCK_SIZE
    .add(DigitalOutput::BLOCK_SIZE)
    .add(SimpleDelay::BLOCK_SIZE)
    .add(DigitalOutput::BLOCK_SIZE)
    ;
  println!("{:?}", SYSTEM_SIZE);
  let storage = HeapSystemStorage::new(SYSTEM_SIZE);
  let mut peripherals = LedSystemPeripherals::new();
  let mut system = LedSystem::new(&storage, &mut peripherals);

  let mut t: f64 = 0.0;
  let step = 0.01;
  let t_print = 0.1;
  let mut t_last_print = 0.0;

  system.init()?;
  println!("t = {:.3}", t);
  println!("{:?}", storage);

  while true {
    let ssi = SystemStateInfo {t: t};
    system.step(&ssi)?;
    std::thread::sleep(std::time::Duration::from_millis((step * 1000.0) as u64));
    // if t - t_last_print >= t_print {
    //   println!("t = {:.3}", t);
    //   println!("{:?}", storage);
    //   t_last_print = t;
    // }
    t += step;
  }

  Ok(())
}