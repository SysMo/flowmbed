use esp_idf_hal::{adc, gpio};
use esp_idf_hal::peripherals::Peripherals;
use flowmbed_core_blocks::{sensors as sensors, sinks as sinks};
use flowmbed_dynsys::core as fds_core;
use flowmbed_dynsys::core::{DynamicalSystem, RequirePeripherals, RequiresStorage};
use flowmbed_esp32::hal::{ADCReader, SerialValueSink};

use flowmbed_core_blocks::cfg_device;

use esp_idf_hal::prelude::*;

struct EspDevPeripherals<'a> {
    __marker: std::marker::PhantomData<&'a ()>,
    adc1: ADCReader<'a, adc::ADC2, gpio::Gpio12, adc::Atten6dB<adc::ADC2>>,
    serial: SerialValueSink,
}

impl<'a> EspDevPeripherals<'a> {
    pub fn new() -> anyhow::Result<EspDevPeripherals<'a>> {
        let peripherals = match Peripherals::take() {
            Some(x) => x,
            None => anyhow::bail!("Peripherals already taken!")
        };
        Ok(EspDevPeripherals {
            __marker: std::marker::PhantomData,
            adc1: ADCReader {
                driver: adc::AdcDriver::new(
                    peripherals.adc2,
                    &adc::config::Config::new()
                        .calibration(true)
                        .resolution(adc::config::Resolution::Resolution9Bit)
                )?,
                channel: adc::AdcChannelDriver::new(peripherals.pins.gpio12)?
            },
            serial: SerialValueSink {},
        })
    }
}

/// Declare circuit structure
struct TempMeasCircuit<'a> {
    sensor_adc: sensors::OneShotAnalog<'a>,
    sink_adc: sinks::FloatSink<'a>,
}

/// Implement circuit structure
impl<'a> TempMeasCircuit<'a> {
    pub fn new<ST: fds_core::DefaultSystemStrorage>(
        storage: &'a ST, peripherals: &'a mut EspDevPeripherals
    ) -> anyhow::Result<TempMeasCircuit<'a>> {
        use fds_core::BlockBuilder;

        let mut builder = fds_core::SystemStorageBuilder::new(storage);

        let mut circuit = TempMeasCircuit {
            sensor_adc: {sensors::OneShotAnalog
                ::builder().sensor(&mut peripherals.adc1).build(&mut builder)},
            sink_adc: {sinks::FloatSink
                ::builder().sink(&mut peripherals.serial).build(&mut builder)},
        };

        circuit.connect()?;
        Ok(circuit)
    }

}

/// Implement DynamicalSystem protocol
impl<'a> DynamicalSystem for TempMeasCircuit<'a> {
    fn connect(&mut self) -> anyhow::Result<()> {
        self.sink_adc.input.connect(&self.sensor_adc.output)?;
        Ok(())
    }

    fn init(&mut self) -> anyhow::Result<()> {
        self.sensor_adc.init()?;
        self.sink_adc.init()?;
        Ok(())
    }

    fn step(&mut self, ssi: &fds_core::SystemStateInfo) -> anyhow::Result<()> {
        self.sensor_adc.step(ssi)?;
        self.sink_adc.step(ssi)?;
        Ok(())
    }
}

/// Implement RequirePeripherals protocol
impl<'a> RequirePeripherals for TempMeasCircuit<'a> {
    type PeripheralsStruct = EspDevPeripherals<'a>;
}

/// Implement RequireStorage protocol
use const_default::ConstDefault;
impl<'a> RequiresStorage for TempMeasCircuit<'a> {
    const SIZE: fds_core::StorageSize =
        fds_core::StorageSize::DEFAULT
            .add(sensors::OneShotAnalog::SIZE)
            .add(sinks::FloatSink::SIZE)
        ;
}

struct MainTask {}

impl MainTask {
    pub fn run() -> anyhow::Result<()> {
        use fds_core::SystemRunner;

        type PeripheralsStruct<'a> = <TempMeasCircuit<'a> as RequirePeripherals>::PeripheralsStruct;
        let storage = fds_core::HeapSystemStorage::new(TempMeasCircuit::SIZE);
        let mut peripherals = PeripheralsStruct::new()?;
        let mut circuit = TempMeasCircuit::new(&storage, &mut peripherals)?;

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
