use esp_idf_hal::{adc, gpio, units};
use esp_idf_hal::peripherals::Peripherals;
use flowmbed_core_blocks::{actuators as actuators, bus as bus, discrete as discrete, sensors as sensors, sinks as sinks, transform as transform};
use flowmbed_dynsys::core as ds_core;
use flowmbed_dynsys::core::{DynamicalSystem, RequirePeripherals, RequiresStorage};
use flowmbed_dynsys::util::containers::{OnceCell, RefOnce};
use flowmbed_esp32::hal as esp32hal;

use flowmbed_core_blocks::cfg_device;
#[allow(unused_imports)]
use flowmbed_dynsys::core::{Float, Int, Bool, String};

#[derive(const_default_derive::ConstDefault)]
struct EspDevPeripherals<'a> {
    __marker: std::marker::PhantomData<&'a ()>,
    __pin: std::marker::PhantomPinned,
    adc1_channel_0: RefOnce<esp32hal::AnalogChannel<'a, gpio::Gpio32, adc::Atten11dB<adc::ADC1>>>,
    adc1_channel_1: RefOnce<esp32hal::AnalogChannel<'a, gpio::Gpio33, adc::Atten11dB<adc::ADC1>>>,
    adc1: RefOnce<esp32hal::AnalogReaderMultiChannel<'a, adc::ADC1, 2>>,
    button1: RefOnce<esp32hal::DigitalInputPin<'a, gpio::Gpio21>>,
    led1: RefOnce<esp32hal::DigitalOutputPin<'a, gpio::Gpio2>>,
    led_pwm_1: RefOnce<esp32hal::PwmMultiChannel<'a, 2>>,
    serial_peripheral: RefOnce<esp32hal::SerialValueSink>,
}

impl<'a> EspDevPeripherals<'a> {

    pub fn init(&mut self) -> anyhow::Result<()> {
        let internal_peripherals = match Peripherals::take() {
            Some(x) => x,
            None => anyhow::bail!("Peripherals already taken!")
        };

        self.adc1_channel_0.init({
            esp32hal::AnalogChannel::new(internal_peripherals.pins.gpio32)?
        })?;
        log::info!("Initialized peripheral {}", stringify!(adc1_channel_0));
        self.adc1_channel_1.init({
            esp32hal::AnalogChannel::new(internal_peripherals.pins.gpio33)?
        })?;
        log::info!("Initialized peripheral {}", stringify!(adc1_channel_1));
        self.adc1.init({
            let adc_channel_config = adc::config::Config::new()
                .calibration(true);

            esp32hal::AnalogReaderMultiChannel {
                driver: adc::AdcDriver::new(
                    internal_peripherals.adc1,
                    &adc_channel_config
                )?,
                channels: [
                    self
                        .adc1_channel_0.mut_ref()?,
                    self
                        .adc1_channel_1.mut_ref()?,
                ]
            }
        })?;
        log::info!("Initialized peripheral {}", stringify!(adc1));
        self.button1.init({
            {let mut driver =
                gpio::PinDriver::input(internal_peripherals.pins.gpio21)?;
            driver.set_pull(gpio::Pull::Up)?;
            driver.into()}
        })?;
        log::info!("Initialized peripheral {}", stringify!(button1));
        self.led1.init({
            gpio::PinDriver::output(internal_peripherals.pins.gpio2)?.into()
        })?;
        log::info!("Initialized peripheral {}", stringify!(led1));
        self.led_pwm_1.init({
            esp32hal::PwmMultiChannel::builder(units::Hertz(1000), internal_peripherals.ledc.timer0)?
            .add_channel(
                internal_peripherals.ledc.channel0,
                internal_peripherals.pins.gpio16)?
            .add_channel(
                internal_peripherals.ledc.channel1,
                internal_peripherals.pins.gpio17)?
            .build()?
        })?;
        log::info!("Initialized peripheral {}", stringify!(led_pwm_1));
        self.serial_peripheral.init({
            esp32hal::SerialValueSink {}
        })?;
        log::info!("Initialized peripheral {}", stringify!(serial_peripheral));
        Ok(())
    }
}

/// Declare circuit structure
#[derive(const_default_derive::ConstDefault)]
struct TempMeasCircuit<'a> {
    sensor_adc: sensors::AnalogReaderMultiChannelBlock<'a, 2>,
    sensor_button: sensors::DigitalReaderBlock<'a>,
    count_trigger: discrete::CountingTrigger<'a>,
    splitter1: bus::ArraySplitter<'a, Float, 2>,
    offset_ch1: transform::Offset<'a>,
    gain_ch1: transform::Gain<'a>,
    offset_ch2: transform::Offset<'a>,
    gain_ch2: transform::Gain<'a>,
    joiner1: bus::ArrayJoiner<'a, Float, 2>,
    pwm1: actuators::PwmMultiChannelBlock<'a, 2>,
    led1: actuators::DigitalOutput<'a>,
    print1: sinks::ArraySink<'a, f32, 2>,
}

/// Implement circuit structure
impl<'a> TempMeasCircuit<'a> {
    pub fn init<ST: ds_core::DefaultSystemStrorage>(
        circuit: &mut OnceCell<TempMeasCircuit<'a>>,
        storage: &'a ST, peripherals: &'a EspDevPeripherals
    ) -> anyhow::Result<()> {
        use ds_core::BlockBuilder;

        let mut builder = ds_core::SystemStorageBuilder::new(storage);

        circuit.set(TempMeasCircuit {
            sensor_adc:{sensors::AnalogReaderMultiChannelBlock
                ::builder().periph_reader(peripherals.adc1.mut_ref()?).build(&mut builder)}
            ,
            sensor_button:{sensors::DigitalReaderBlock
                ::builder().periph_reader(peripherals.button1.mut_ref()?).build(&mut builder)}
            ,
            count_trigger:{discrete::CountingTrigger
                ::builder().pulses_up(1).pulses_down(2).initial_count(0).build(&mut builder)}
            ,
            splitter1:{bus::ArraySplitter
                ::builder().build(&mut builder)}
            ,
            offset_ch1:{transform::Offset
                ::builder().offset(-0.142).build(&mut builder)}
            ,
            gain_ch1:{transform::Gain
                ::builder().gain(0.3).build(&mut builder)}
            ,
            offset_ch2:{transform::Offset
                ::builder().offset(-0.142).build(&mut builder)}
            ,
            gain_ch2:{transform::Gain
                ::builder().gain(0.3).build(&mut builder)}
            ,
            joiner1:{bus::ArrayJoiner
                ::builder().build(&mut builder)}
            ,
            pwm1:{actuators::PwmMultiChannelBlock
                ::builder().periph_out(peripherals.led_pwm_1.mut_ref()?).build(&mut builder)}
            ,
            led1:{actuators::DigitalOutput
                ::builder().periph_out(peripherals.led1.mut_ref()?).build(&mut builder)}
            ,
            print1:{sinks::ArraySink
                ::builder().sink(peripherals.serial_peripheral.mut_ref()?).build(&mut builder)}
            ,
        })?;

        circuit.get_mut()?.connect()?;
        Ok(())
    }

}

/// Implement DynamicalSystem protocol
impl<'a> DynamicalSystem<'a> for TempMeasCircuit<'a> {
    fn connect(&mut self) -> anyhow::Result<()> {
        self.count_trigger.input.connect(&self.sensor_button.output)?;
        self.splitter1.input.connect(&self.sensor_adc.readings)?;
        self.offset_ch1.input.connect(&self.splitter1.outputs[0])?;
        self.gain_ch1.input.connect(&self.offset_ch1.output)?;
        self.offset_ch2.input.connect(&self.splitter1.outputs[1])?;
        self.gain_ch2.input.connect(&self.offset_ch2.output)?;
        self.joiner1.inputs[0].connect(&self.gain_ch1.output)?;
        self.joiner1.inputs[1].connect(&self.gain_ch2.output)?;
        self.pwm1.duty.connect(&self.joiner1.output)?;
        self.led1.input.connect(&self.count_trigger.output)?;
        self.print1.input.connect(&self.joiner1.output)?;
        Ok(())
    }

    fn init(&mut self) -> anyhow::Result<()> {
        self.sensor_adc.init()?;
        self.sensor_button.init()?;
        self.count_trigger.init()?;
        self.splitter1.init()?;
        self.offset_ch1.init()?;
        self.gain_ch1.init()?;
        self.offset_ch2.init()?;
        self.gain_ch2.init()?;
        self.joiner1.init()?;
        self.pwm1.init()?;
        self.led1.init()?;
        self.print1.init()?;
        Ok(())
    }

    fn step(&mut self, ssi: &ds_core::SystemStateInfo) -> anyhow::Result<()> {
        self.sensor_adc.step(ssi)?;
        self.sensor_button.step(ssi)?;
        self.count_trigger.step(ssi)?;
        self.splitter1.step(ssi)?;
        self.offset_ch1.step(ssi)?;
        self.gain_ch1.step(ssi)?;
        self.offset_ch2.step(ssi)?;
        self.gain_ch2.step(ssi)?;
        self.joiner1.step(ssi)?;
        self.pwm1.step(ssi)?;
        self.led1.step(ssi)?;
        self.print1.step(ssi)?;
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
    const SIZE: ds_core::StorageSize =
        ds_core::StorageSize::DEFAULT
            .add(sensors::AnalogReaderMultiChannelBlock::<2>::SIZE)
            .add(sensors::DigitalReaderBlock::SIZE)
            .add(discrete::CountingTrigger::SIZE)
            .add(bus::ArraySplitter::<Float, 2>::SIZE)
            .add(transform::Offset::SIZE)
            .add(transform::Gain::SIZE)
            .add(transform::Offset::SIZE)
            .add(transform::Gain::SIZE)
            .add(bus::ArrayJoiner::<Float, 2>::SIZE)
            .add(actuators::PwmMultiChannelBlock::<2>::SIZE)
            .add(actuators::DigitalOutput::SIZE)
            .add(sinks::ArraySink::<f32, 2>::SIZE)
        ;
}

struct MainTask {}

impl MainTask {
    pub fn run<'a>() -> anyhow::Result<()> {
        use ds_core::SystemRunner;

        type PeripheralsStruct<'a> = <TempMeasCircuit<'a> as RequirePeripherals>::PeripheralsStruct;
        let storage = ds_core::HeapSystemStorage::new(TempMeasCircuit::SIZE);
        let mut peripherals: PeripheralsStruct = PeripheralsStruct::DEFAULT;
        peripherals.init()?;

        let mut circuit: OnceCell<TempMeasCircuit> = OnceCell::DEFAULT;
        TempMeasCircuit::init(&mut circuit, &storage, &mut peripherals)?;

        let run_settings = ds_core::FixedStepRunSettings {
            t_step: 0.100000,
            speedup: 1.000000,
            t_end: None,
            ..Default::default()
        };
        let mut runner = ds_core::FixedStepRunner::new(run_settings);

        runner.init(circuit.get_mut()?)?;
        runner.run(circuit.get_mut()?)?;

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
