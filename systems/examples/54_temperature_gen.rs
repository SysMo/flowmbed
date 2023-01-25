use esp_idf_hal::{adc, gpio};
use esp_idf_hal::peripherals::Peripherals;
use flowmbed_core_blocks::{actuators as actuators, discrete as discrete, sensors as sensors, sinks as sinks};
use flowmbed_dynsys::core as fds_core;
use flowmbed_dynsys::core::{DynamicalSystem, RequirePeripherals, RequiresStorage};
use flowmbed_esp32::hal as esp32hal;
use flowmbed_esp32::hal::ADCReader;

use flowmbed_core_blocks::cfg_device;

use esp_idf_hal::prelude::*;

struct EspDevPeripherals<'a> {
    __marker: std::marker::PhantomData<&'a ()>,
    adc1: ADCReader<'a, adc::ADC1, gpio::Gpio33, adc::Atten11dB<adc::ADC1>>,
    button1: esp32hal::DigitalInputPin<'a, gpio::Gpio16>,
    led1: esp32hal::DigitalOutputPin<'a, gpio::Gpio2>,
    serial1: esp32hal::SerialValueSink,
    serial2: esp32hal::SerialValueSink,
}

impl<'a> EspDevPeripherals<'a> {
    pub fn new() -> anyhow::Result<EspDevPeripherals<'a>> {
        let peripherals = match Peripherals::take() {
            Some(x) => x,
            None => anyhow::bail!("Peripherals already taken!")
        };

        #[allow(unused_mut)]
        let mut adc1 = ADCReader {
            driver: adc::AdcDriver::new(
                peripherals.adc1,
                &adc::config::Config::new()
                    .calibration(true)
                    .resolution(adc::config::Resolution::Resolution10Bit)
            )?,
            channel: adc::AdcChannelDriver::new(peripherals.pins.gpio33)?
        };
        #[allow(unused_mut)]
        let mut button1 = gpio::PinDriver::input(peripherals.pins.gpio16)?;
        button1.set_pull(gpio::Pull::Up)?;
        #[allow(unused_mut)]
        let mut led1 = gpio::PinDriver::output(peripherals.pins.gpio2)?;
        #[allow(unused_mut)]
        let mut serial1 = esp32hal::SerialValueSink {};
        #[allow(unused_mut)]
        let mut serial2 = esp32hal::SerialValueSink {};

        Ok(EspDevPeripherals {
            __marker: std::marker::PhantomData,
            adc1: adc1.into(),
            button1: button1.into(),
            led1: led1.into(),
            serial1: serial1.into(),
            serial2: serial2.into(),
        })
    }
}

/// Declare circuit structure
struct TempMeasCircuit<'a> {
    sensor_adc: sensors::AnalogReaderBlock<'a>,
    sensor_button: sensors::DigitalReaderBlock<'a>,
    count_trigger: discrete::CountingTrigger<'a>,
    print1: sinks::FloatSink<'a>,
    print2: sinks::FloatSink<'a>,
    led1: actuators::DigitalOutput<'a>,
}

/// Implement circuit structure
impl<'a> TempMeasCircuit<'a> {
    pub fn new<ST: fds_core::DefaultSystemStrorage>(
        storage: &'a ST, peripherals: &'a mut EspDevPeripherals
    ) -> anyhow::Result<TempMeasCircuit<'a>> {
        use fds_core::BlockBuilder;

        let mut builder = fds_core::SystemStorageBuilder::new(storage);

        let mut circuit = TempMeasCircuit {
            sensor_adc: {sensors::AnalogReaderBlock
                ::builder().sensor(&mut peripherals.adc1).build(&mut builder)},
            sensor_button: {sensors::DigitalReaderBlock
                ::builder().sensor(&mut peripherals.button1).build(&mut builder)},
            count_trigger: {discrete::CountingTrigger
                ::builder().initial_count(0).pulses_up(1).pulses_down(2).build(&mut builder)},
            print1: {sinks::FloatSink
                ::builder().sink(&mut peripherals.serial1).build(&mut builder)},
            print2: {sinks::FloatSink
                ::builder().sink(&mut peripherals.serial2).build(&mut builder)},
            led1: {actuators::DigitalOutput
                ::builder().output(&mut peripherals.led1).build(&mut builder)},
        };

        circuit.connect()?;
        Ok(circuit)
    }

}

/// Implement DynamicalSystem protocol
impl<'a> DynamicalSystem for TempMeasCircuit<'a> {
    fn connect(&mut self) -> anyhow::Result<()> {
        self.count_trigger.input.connect(&self.sensor_button.output)?;
        self.led1.input.connect(&self.count_trigger.output)?;
        self.print1.input.connect(&self.sensor_adc.output)?;
        self.print2.input.connect(&self.sensor_adc.output)?;
        Ok(())
    }

    fn init(&mut self) -> anyhow::Result<()> {
        self.sensor_adc.init()?;
        self.sensor_button.init()?;
        self.count_trigger.init()?;
        self.print1.init()?;
        self.print2.init()?;
        self.led1.init()?;
        Ok(())
    }

    fn step(&mut self, ssi: &fds_core::SystemStateInfo) -> anyhow::Result<()> {
        self.sensor_adc.step(ssi)?;
        self.sensor_button.step(ssi)?;
        self.count_trigger.step(ssi)?;
        self.print1.step(ssi)?;
        self.print2.step(ssi)?;
        self.led1.step(ssi)?;
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
            .add(sensors::AnalogReaderBlock::SIZE)
            .add(sensors::DigitalReaderBlock::SIZE)
            .add(discrete::CountingTrigger::SIZE)
            .add(sinks::FloatSink::SIZE)
            .add(sinks::FloatSink::SIZE)
            .add(actuators::DigitalOutput::SIZE)
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
