use esp32_hal::gpio;
use esp32_hal::peripherals::Peripherals;
use flowmbed_core_blocks::{discrete as discrete, hardware_sinks as hardware_sinks, sources as sources};
use flowmbed_dynsys::core as fds_core;
use flowmbed_dynsys::core::{DynamicalSystem, RequirePeripherals, RequiresStorage};

use flowmbed_core_blocks::cfg_device;
use flowmbed_core_blocks::hal::esp32_hal;

use esp_idf_hal::prelude::*;
/// Device LedEsp
struct LedEspPeripherals<'a> {
    __marker: std::marker::PhantomData<&'a ()>,
    led1: gpio::PinDriver::<'a, gpio::Gpio15, gpio::Output>,
    led2: gpio::PinDriver::<'a, gpio::Gpio2, gpio::Output>,
    led3: gpio::PinDriver::<'a, gpio::Gpio4, gpio::Output>,
    led4: gpio::PinDriver::<'a, gpio::Gpio16, gpio::Output>,
    led5: gpio::PinDriver::<'a, gpio::Gpio17, gpio::Output>,
    led6: gpio::PinDriver::<'a, gpio::Gpio5, gpio::Output>,
}

impl<'a> LedEspPeripherals<'a> {
    pub fn new() -> anyhow::Result<LedEspPeripherals<'a>> {
        let device_peripherals = match Peripherals::take() {
            Some(x) => x,
            None => anyhow::bail!("Peripherals already taken!")
        };
        Ok(LedEspPeripherals {
            __marker: std::marker::PhantomData,
            led1: gpio::PinDriver::output(device_peripherals.pins.gpio15)?,
            led2: gpio::PinDriver::output(device_peripherals.pins.gpio2)?,
            led3: gpio::PinDriver::output(device_peripherals.pins.gpio4)?,
            led4: gpio::PinDriver::output(device_peripherals.pins.gpio16)?,
            led5: gpio::PinDriver::output(device_peripherals.pins.gpio17)?,
            led6: gpio::PinDriver::output(device_peripherals.pins.gpio5)?,
        })
    }
}

/// Declare circuit structure
struct LedCircuit<'a> {
    source: sources::SquareWaveSource<'a>,
    trigger1: discrete::CountingTrigger<'a>,
    trigger2: discrete::CountingTrigger<'a>,
    trigger3: discrete::CountingTrigger<'a>,
    trigger4: discrete::CountingTrigger<'a>,
    trigger5: discrete::CountingTrigger<'a>,
    trigger6: discrete::CountingTrigger<'a>,
    led1: hardware_sinks::DigitalOutput<'a>,
    led2: hardware_sinks::DigitalOutput<'a>,
    led3: hardware_sinks::DigitalOutput<'a>,
    led4: hardware_sinks::DigitalOutput<'a>,
    led5: hardware_sinks::DigitalOutput<'a>,
    led6: hardware_sinks::DigitalOutput<'a>,
}

/// Implement circuit structure
impl<'a> LedCircuit<'a> {
    pub fn new<ST: fds_core::DefaultSystemStrorage>(
        storage: &'a ST, peripherals: &'a mut LedEspPeripherals
    ) -> anyhow::Result<LedCircuit<'a>> {
        use fds_core::BlockBuilder;

        let mut builder = fds_core::SystemStorageBuilder::new(storage);

        let mut circuit = LedCircuit {
            source: {sources::SquareWaveSource
                ::builder().period(0.1).build(&mut builder)},
            trigger1: {discrete::CountingTrigger
                ::builder().pulses_down(6).initial_count(0).pulses_up(1).build(&mut builder)},
            trigger2: {discrete::CountingTrigger
                ::builder().initial_count(0).pulses_up(2).pulses_down(5).build(&mut builder)},
            trigger3: {discrete::CountingTrigger
                ::builder().pulses_down(4).initial_count(0).pulses_up(3).build(&mut builder)},
            trigger4: {discrete::CountingTrigger
                ::builder().pulses_up(4).initial_count(0).pulses_down(3).build(&mut builder)},
            trigger5: {discrete::CountingTrigger
                ::builder().pulses_down(2).initial_count(0).pulses_up(5).build(&mut builder)},
            trigger6: {discrete::CountingTrigger
                ::builder().initial_count(0).pulses_up(6).pulses_down(1).build(&mut builder)},
            led1: {hardware_sinks::DigitalOutput
                ::builder().out(&mut peripherals.led1).build(&mut builder)},
            led2: {hardware_sinks::DigitalOutput
                ::builder().out(&mut peripherals.led2).build(&mut builder)},
            led3: {hardware_sinks::DigitalOutput
                ::builder().out(&mut peripherals.led3).build(&mut builder)},
            led4: {hardware_sinks::DigitalOutput
                ::builder().out(&mut peripherals.led4).build(&mut builder)},
            led5: {hardware_sinks::DigitalOutput
                ::builder().out(&mut peripherals.led5).build(&mut builder)},
            led6: {hardware_sinks::DigitalOutput
                ::builder().out(&mut peripherals.led6).build(&mut builder)},
        };

        circuit.connect()?;
        Ok(circuit)
    }

}

/// Implement DynamicalSystem protocol
impl<'a> DynamicalSystem for LedCircuit<'a> {
    fn connect(&mut self) -> anyhow::Result<()> {
        self.trigger1.input.connect(&self.source.output)?;
        self.led1.input.connect(&self.trigger1.output)?;
        self.trigger2.input.connect(&self.source.output)?;
        self.led2.input.connect(&self.trigger2.output)?;
        self.trigger3.input.connect(&self.source.output)?;
        self.led3.input.connect(&self.trigger3.output)?;
        self.trigger4.input.connect(&self.source.output)?;
        self.led4.input.connect(&self.trigger4.output)?;
        self.trigger5.input.connect(&self.source.output)?;
        self.led5.input.connect(&self.trigger5.output)?;
        self.trigger6.input.connect(&self.source.output)?;
        self.led6.input.connect(&self.trigger6.output)?;
        Ok(())
    }

    fn init(&mut self) -> anyhow::Result<()> {
        self.source.init()?;
        self.trigger1.init()?;
        self.trigger2.init()?;
        self.trigger3.init()?;
        self.trigger4.init()?;
        self.trigger5.init()?;
        self.trigger6.init()?;
        self.led1.init()?;
        self.led2.init()?;
        self.led3.init()?;
        self.led4.init()?;
        self.led5.init()?;
        self.led6.init()?;
        Ok(())
    }

    fn step(&mut self, ssi: &fds_core::SystemStateInfo) -> anyhow::Result<()> {
        self.source.step(ssi)?;
        self.trigger1.step(ssi)?;
        self.trigger2.step(ssi)?;
        self.trigger3.step(ssi)?;
        self.trigger4.step(ssi)?;
        self.trigger5.step(ssi)?;
        self.trigger6.step(ssi)?;
        self.led1.step(ssi)?;
        self.led2.step(ssi)?;
        self.led3.step(ssi)?;
        self.led4.step(ssi)?;
        self.led5.step(ssi)?;
        self.led6.step(ssi)?;
        Ok(())
    }
}

/// Implement RequirePeripherals protocol
impl<'a> RequirePeripherals for LedCircuit<'a> {
    type PeripheralsStruct = LedEspPeripherals<'a>;
}

/// Implement RequireStorage protocol
use const_default::ConstDefault;
impl<'a> RequiresStorage for LedCircuit<'a> {
    const SIZE: fds_core::StorageSize =
        fds_core::StorageSize::DEFAULT
            .add(sources::SquareWaveSource::SIZE)
            .add(discrete::CountingTrigger::SIZE)
            .add(discrete::CountingTrigger::SIZE)
            .add(discrete::CountingTrigger::SIZE)
            .add(discrete::CountingTrigger::SIZE)
            .add(discrete::CountingTrigger::SIZE)
            .add(discrete::CountingTrigger::SIZE)
            .add(hardware_sinks::DigitalOutput::SIZE)
            .add(hardware_sinks::DigitalOutput::SIZE)
            .add(hardware_sinks::DigitalOutput::SIZE)
            .add(hardware_sinks::DigitalOutput::SIZE)
            .add(hardware_sinks::DigitalOutput::SIZE)
            .add(hardware_sinks::DigitalOutput::SIZE)
        ;
}

struct MainTask {}

impl MainTask {
    pub fn run() -> anyhow::Result<()> {
        use fds_core::SystemRunner;

        type PeripheralsStruct<'a> = <LedCircuit<'a> as RequirePeripherals>::PeripheralsStruct;
        let storage = fds_core::HeapSystemStorage::new(LedCircuit::SIZE);
        let mut peripherals = PeripheralsStruct::new()?;
        let mut circuit = LedCircuit::new(&storage, &mut peripherals)?;

        let run_settings = fds_core::FixedStepRunSettings {
            t_step: 0.010000,
            speedup: 1.000000,
            t_end: None,
            ..Default::default()
        };
        let mut runner = fds_core::FixedStepRunner::new(&mut circuit, run_settings);

        runner.init()?;
        runner.run()?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    // Configure logging
    cfg_device::config_logger();
    // Start the main task
    MainTask::run()?;

    Ok(())
}
