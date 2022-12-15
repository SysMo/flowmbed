extern crate flowmbed_shared;


use flowmbed_shared::dynsys::system::SystemStateInfo;
use flowmbed_shared::dynsys::{StorageSize, DefaultSystemStrorage, SystemStorageBuilder};
use flowmbed_shared::dynsys::HeapSystemStorage;
use flowmbed_shared::dynsys::Block;
use flowmbed_shared::dynsys::block_library::{
  sources::SquareSource,
  hardware_sinks::{DummyDigitalOutput, DigitalOutput}
};
use embedded_hal::digital;
use std::cell::RefCell;


struct LedSystemPeripherals {
  led: RefCell<Box<dyn digital::OutputPin<Error = anyhow::Error>>>
}

impl LedSystemPeripherals {
  pub fn new() -> LedSystemPeripherals {
    LedSystemPeripherals {
      led: RefCell::new(Box::new(DummyDigitalOutput {}))
    }
  }
}

struct LedSystem<'a> {
  source: SquareSource<'a>,
  sink: DigitalOutput<'a>
}

impl<'a> LedSystem<'a> {
  pub fn new<ST: DefaultSystemStrorage>(storage: &'a ST, peripherals: &'a LedSystemPeripherals) -> LedSystem<'a> {
    let mut builder = SystemStorageBuilder::new(storage);

    let mut blocks = LedSystem {
      source: SquareSource::new(&mut builder),
      sink: DigitalOutput::new(&mut builder, peripherals.led.borrow_mut()),
    };

    blocks.sink.input.connect(&blocks.source.output);

    blocks
  }

  pub fn init(&mut self) -> anyhow::Result<()> {
    Ok(())
  }

  pub fn step(&mut self, ssi: &SystemStateInfo) -> anyhow::Result<()> {
    self.source.compute(ssi)?;
    self.sink.compute(ssi)?;
    Ok(())
  }

}

fn main() -> anyhow::Result<()> {
  const size : StorageSize = 
    SquareSource::size.add(DigitalOutput::size);
  let storage = 
    HeapSystemStorage::new(size);
  let peripherals = LedSystemPeripherals::new();

  let mut system = LedSystem::new(&storage, &peripherals);
  let mut t: u64 = 0;
  let step = 1;

  system.init();

  while t <= 10 {
    let ssi = SystemStateInfo {t: (t as f64) * 1000.0};
    system.step(&ssi)?;
    std::thread::sleep(std::time::Duration::from_secs(step) / 5);
    t += step;
  }

  Ok(())
}