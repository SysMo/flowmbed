use flowmbed_peripherals::sensors::traits::AnalogReader;
use flowmbed_peripherals::actuators::traits::PwmChannel;

pub struct BioreactorPeripherals<'a> {
  pub thermal: ThermalPeripherals,
  pub stirrer: StirrerPeripherals<'a>,
}

pub struct ThermalPeripherals {
}

pub struct StirrerPeripherals<'a> {
  pub speed_reader: &'a mut dyn AnalogReader,
  pub speed_pwm: &'a mut dyn PwmChannel,
}