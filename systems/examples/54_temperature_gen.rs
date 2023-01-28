use esp_idf_hal::{adc, gpio};
use esp_idf_hal::peripherals::Peripherals;
use flowmbed_core_blocks::{actuators as actuators, discrete as discrete, sensors as sensors, sinks as sinks};
use flowmbed_dynsys::core as fds_core;
use flowmbed_dynsys::core::{DynamicalSystem, RequirePeripherals, RequiresStorage};
use flowmbed_dynsys::util::containers::RefOnce;
use flowmbed_esp32::hal as esp32hal;

use flowmbed_core_blocks::cfg_device;

#[derive(const_default_derive::ConstDefault)]
struct EspDevPeripherals<'a> {
    __marker: std::marker::PhantomData<&'a ()>,
    channel_0: RefOnce<esp32hal::AnalogChannel<'a, gpio::Gpio32, adc::Atten11dB<adc::ADC1>>>,
    channel_1: RefOnce<esp32hal::AnalogChannel<'a, gpio::Gpio33, adc::Atten11dB<adc::ADC1>>>,
    adc1: RefOnce<esp32hal::AnalogReaderMultiChannel<'a, adc::ADC1, 2>>,
    button1: RefOnce<esp32hal::DigitalInputPin<'a, gpio::Gpio16>>,
    led1: RefOnce<esp32hal::DigitalOutputPin<'a, gpio::Gpio2>>,
    serial1: RefOnce<esp32hal::SerialValueSink>,
    serial2: RefOnce<esp32hal::SerialValueSink>,
}

impl<'a> EspDevPeripherals<'a> {
    pub fn new() -> anyhow::Result<&'static EspDevPeripherals<'static>> {
        static MCU_PERIPHERALS: EspDevPeripherals<'static> = EspDevPeripherals::DEFAULT;

        let peripherals = match Peripherals::take() {
            Some(x) => x,
            None => anyhow::bail!("Peripherals already taken!")
        };

        MCU_PERIPHERALS.channel_0.init({
            esp32hal::AnalogChannel::new(peripherals.pins.gpio32)?
        })?;

        MCU_PERIPHERALS.channel_1.init({
            esp32hal::AnalogChannel::new(peripherals.pins.gpio33)?
        })?;

        MCU_PERIPHERALS.adc1.init({
            let adc_channel_config = adc::config::Config::new()
                .calibration(true);

            esp32hal::AnalogReaderMultiChannel {
                driver: adc::AdcDriver::new(
                    peripherals.adc1,
                    &adc_channel_config
                )?,
                channels: [
                    MCU_PERIPHERALS.channel_0.mut_ref()?,
                    MCU_PERIPHERALS.channel_1.mut_ref()?,
                ]
            }
        })?;

        MCU_PERIPHERALS.button1.init({
            {let mut driver =
                gpio::PinDriver::input(peripherals.pins.gpio16)?;
            driver.set_pull(gpio::Pull::Up)?;
            driver.into()}
        })?;

        MCU_PERIPHERALS.led1.init({
            gpio::PinDriver::output(peripherals.pins.gpio2)?.into()
        })?;

        MCU_PERIPHERALS.serial1.init({
            esp32hal::SerialValueSink {}
        })?;

        MCU_PERIPHERALS.serial2.init({
            esp32hal::SerialValueSink {}
        })?;

        Ok(&MCU_PERIPHERALS)
    }
}

/// Declare circuit structure
struct TempMeasCircuit<'a> {
    sensor_adc: sensors::AnalogReaderMultiChannelBlock<'a>,
    sensor_button: sensors::DigitalReaderBlock<'a>,
    count_trigger: discrete::CountingTrigger<'a>,
    print1: sinks::FloatSink<'a>,
    print2: sinks::FloatSink<'a>,
    led1: actuators::DigitalOutput<'a>,
}

/// Implement circuit structure
impl<'a> TempMeasCircuit<'a> {
    pub fn new<ST: fds_core::DefaultSystemStrorage>(
        storage: &'a ST, peripherals: &'a EspDevPeripherals
    ) -> anyhow::Result<TempMeasCircuit<'a>> {
        use fds_core::BlockBuilder;

        let mut builder = fds_core::SystemStorageBuilder::new(storage);

        let mut circuit = TempMeasCircuit {
            sensor_adc: {sensors::AnalogReaderMultiChannelBlock
                ::builder().sensor(peripherals.adc1.mut_ref()?).build(&mut builder)},
            sensor_button: {sensors::DigitalReaderBlock
                ::builder().sensor(peripherals.button1.mut_ref()?).build(&mut builder)},
            count_trigger: {discrete::CountingTrigger
                ::builder().pulses_up(1).pulses_down(2).initial_count(0).build(&mut builder)},
            print1: {sinks::FloatSink
                ::builder().sink(peripherals.serial1.mut_ref()?).build(&mut builder)},
            print2: {sinks::FloatSink
                ::builder().sink(peripherals.serial2.mut_ref()?).build(&mut builder)},
            led1: {actuators::DigitalOutput
                ::builder().output(peripherals.led1.mut_ref()?).build(&mut builder)},
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
        self.print1.input.connect(&self.sensor_adc.output1)?;
        self.print2.input.connect(&self.sensor_adc.output2)?;
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
            .add(sensors::AnalogReaderMultiChannelBlock::SIZE)
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
            t_step: 0.100000,
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
