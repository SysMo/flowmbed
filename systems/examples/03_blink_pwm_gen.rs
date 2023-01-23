use esp32_hal::ledc;
use esp32_hal::peripherals::Peripherals;
use flowmbed_core_blocks::{actuators as hardware_sinks, sources as sources};
use flowmbed_dynsys::core as fds_core;
use flowmbed_dynsys::core::{DynamicalSystem, RequirePeripherals, RequiresStorage};

use flowmbed_core_blocks::cfg_device;
use flowmbed_core_blocks::hal::esp32_hal;

use esp_idf_hal::prelude::*;
/// Device LedEsp
struct LedEspPeripherals<'a> {
    __marker: std::marker::PhantomData<&'a ()>,
    led1: ledc::LedcDriver::<'a>,
    led2: ledc::LedcDriver::<'a>,
    led3: ledc::LedcDriver::<'a>,
}

impl<'a> LedEspPeripherals<'a> {
    pub fn new() -> anyhow::Result<LedEspPeripherals<'a>> {
        let device_peripherals = match Peripherals::take() {
            Some(x) => x,
            None => anyhow::bail!("Peripherals already taken!")
        };
        Ok(LedEspPeripherals {
            __marker: std::marker::PhantomData,
            led1: ledc::LedcDriver::new(
                device_peripherals.ledc.channel0,
                ledc::LedcTimerDriver::new(
                    device_peripherals.ledc.timer0,
                        &ledc::config::TimerConfig::new().frequency(1000_u32.Hz().into()),
                )?,
                device_peripherals.pins.gpio15,
            )?,
            led2: ledc::LedcDriver::new(
                device_peripherals.ledc.channel1,
                ledc::LedcTimerDriver::new(
                    device_peripherals.ledc.timer1,
                        &ledc::config::TimerConfig::new().frequency(1000_u32.Hz().into()),
                )?,
                device_peripherals.pins.gpio2,
            )?,
            led3: ledc::LedcDriver::new(
                device_peripherals.ledc.channel2,
                ledc::LedcTimerDriver::new(
                    device_peripherals.ledc.timer2,
                        &ledc::config::TimerConfig::new().frequency(1000_u32.Hz().into()),
                )?,
                device_peripherals.pins.gpio4,
            )?,
        })
    }
}

/// Declare circuit structure
struct LedCircuit<'a> {
    source1: sources::SineWaveSource<'a>,
    source2: sources::SineWaveSource<'a>,
    source3: sources::SineWaveSource<'a>,
    pwm1: actuators::PWMOutput<'a>,
    pwm2: actuators::PWMOutput<'a>,
    pwm3: actuators::PWMOutput<'a>,
}

/// Implement circuit structure
impl<'a> LedCircuit<'a> {
    pub fn new<ST: fds_core::DefaultSystemStrorage>(
        storage: &'a ST, peripherals: &'a mut LedEspPeripherals
    ) -> anyhow::Result<LedCircuit<'a>> {
        use fds_core::BlockBuilder;

        let mut builder = fds_core::SystemStorageBuilder::new(storage);

        let mut circuit = LedCircuit {
            source1: {sources::SineWaveSource
                ::builder().amplitude(0.5).period(1.1).offset(0.5).build(&mut builder)},
            source2: {sources::SineWaveSource
                ::builder().offset(0.5).period(2.2).amplitude(0.5).build(&mut builder)},
            source3: {sources::SineWaveSource
                ::builder().amplitude(0.5).period(4.4).offset(0.5).build(&mut builder)},
            pwm1: {actuators::PWMOutput
                ::builder().out(&mut peripherals.led1).build(&mut builder)},
            pwm2: {actuators::PWMOutput
                ::builder().out(&mut peripherals.led2).build(&mut builder)},
            pwm3: {actuators::PWMOutput
                ::builder().out(&mut peripherals.led3).build(&mut builder)},
        };

        circuit.connect()?;
        Ok(circuit)
    }

}

/// Implement DynamicalSystem protocol
impl<'a> DynamicalSystem for LedCircuit<'a> {
    fn connect(&mut self) -> anyhow::Result<()> {
        self.pwm1.duty.connect(&self.source1.output)?;
        self.pwm2.duty.connect(&self.source2.output)?;
        self.pwm3.duty.connect(&self.source3.output)?;
        Ok(())
    }

    fn init(&mut self) -> anyhow::Result<()> {
        self.source1.init()?;
        self.source2.init()?;
        self.source3.init()?;
        self.pwm1.init()?;
        self.pwm2.init()?;
        self.pwm3.init()?;
        Ok(())
    }

    fn step(&mut self, ssi: &fds_core::SystemStateInfo) -> anyhow::Result<()> {
        self.source1.step(ssi)?;
        self.source2.step(ssi)?;
        self.source3.step(ssi)?;
        self.pwm1.step(ssi)?;
        self.pwm2.step(ssi)?;
        self.pwm3.step(ssi)?;
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
            .add(sources::SineWaveSource::SIZE)
            .add(sources::SineWaveSource::SIZE)
            .add(sources::SineWaveSource::SIZE)
            .add(actuators::PWMOutput::SIZE)
            .add(actuators::PWMOutput::SIZE)
            .add(actuators::PWMOutput::SIZE)
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
