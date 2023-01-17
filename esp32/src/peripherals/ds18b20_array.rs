use embedded_hal_0_2::blocking::delay::{DelayUs, DelayMs};
use embedded_hal_0_2::digital::v2::{InputPin, OutputPin};
use ds18b20::{Ds18b20, read_scratchpad, SensorData};
use one_wire_bus::{OneWire, OneWireResult, OneWireError, Address};
use log::*;
use core::fmt::{Debug, Display};
use core::num;
pub use ds18b20::Resolution;
trait IntoAnyhow<V, E> where E: std::fmt::Debug {
  fn into_anyhow(self) -> anyhow::Result<V>;
}

impl<V, E> IntoAnyhow<V, E> for OneWireResult<V, E> 
where E: std::fmt::Debug {
  fn into_anyhow(self) -> anyhow::Result<V> {
    match self {
      Ok(v) => Ok(v),
      Err(e) => Err(anyhow::anyhow!("{:?}", e)),
    }
  }
}

pub struct DS18B20Array<P, E, D>
where P: OutputPin<Error = E> + InputPin<Error = E>, 
      E: Debug + Display + std::marker::Sync,
      D: DelayUs<u16> + DelayMs<u16>
{
  bus: OneWire<P>,
  delay: D,
  sensor: Ds18b20,
  resolution: Resolution,
  alarm_high: i8,
  alarm_low: i8,
}

impl<P, E, D> DS18B20Array<P, E, D> 
where P: OutputPin<Error = E> + InputPin<Error = E>, 
      E: Debug + Display + std::marker::Sync,
      D: DelayUs<u16> + DelayMs<u16>
{
  pub fn new(pin: P, mut delay: D) -> anyhow::Result<Self> {
    let mut bus = OneWire::new(pin).into_anyhow()?;
    let device_address = Self::find_devices(&mut bus, &mut delay)?;
    // You will generally create the sensor once, and save it for later
    let sensor = Ds18b20::new::<E>(device_address).into_anyhow()?;
    let mut device_array = DS18B20Array {
      bus, delay, sensor, resolution: Resolution::Bits12, 
      alarm_high: 100, alarm_low: 0
    };
    device_array.write_settings()?;
    Ok(device_array)
  }

  // fn read_config(&mut self) -> anyhow::Result<()> {
  //   let mut scratchpad = read_scratchpad(
  //     self.sensor.address(), &mut self.bus, &mut self.delay
  //   ).into_anyhow()?;    
  //   info!("{:?}", scratchpad);
  //   self.alarm_high = scratchpad[2];
  //   self.alarm_low = scratchpad[3];

  //   Ok(())
  // }
  pub fn set_resolution(&mut self, r: Resolution) -> anyhow::Result<()> {
    self.resolution = r;
    self.write_settings()
  }

  pub fn write_settings(&mut self) -> anyhow::Result<()> {
    self.sensor.set_config(self.alarm_low, self.alarm_high, self.resolution, 
      &mut self.bus, &mut self.delay
    ).into_anyhow()?;
    Ok(())
  }

  fn find_devices(one: &mut OneWire<P>, delay: &mut D) -> anyhow::Result<Address> {
    let mut num_found = 0;
    let mut result: anyhow::Result<Address> = Err(anyhow::anyhow!("No DS18b20 device found"));

    for device_address in one.devices(false, delay) {
      // The search could fail at any time, so check each result. The iterator automatically
      // ends after an error.
      let device_address = device_address.into_anyhow()?;

      if device_address.family_code() != ds18b20::FAMILY_CODE {
        // skip other devices
        continue;
      }

      // The family code can be used to identify the type of device
      // If supported, another crate can be used to interact with that device at the given address
      info!("Found device at address {:?} with family code: {:#x?}",
              device_address, device_address.family_code());

      num_found += 1;
      result = Ok(device_address);
    }
    
    if num_found > 1 {
      result = Err(anyhow::anyhow!("More than one device found on the bus"));
    }

    return result
  }

  fn read_data(&mut self) -> anyhow::Result<SensorData> {
    // let mut scratchpad = read_scratchpad(
    //   self.sensor.address(), &mut self.bus, &mut self.delay
    // ).into_anyhow()?;    
    // info!("{:?}", scratchpad);
    // self.alarm_high = scratchpad[2];
    // self.alarm_low = scratchpad[3];
    let scratchpad = read_scratchpad(
      self.sensor.address(), &mut self.bus, &mut self.delay
    ).into_anyhow()?;

    let resolution = match scratchpad[4] {
      0b00011111 => Resolution::Bits9,
      0b00111111 => Resolution::Bits10,
      0b01011111 => Resolution::Bits11,
      0b01111111 => Resolution::Bits12,
      _ => return Err(OneWireError::<E>::CrcMismatch).into_anyhow(),
    };

    let raw_temp = u16::from_le_bytes([scratchpad[0], scratchpad[1]]);
    let temperature = match resolution {
        Resolution::Bits12 => (raw_temp as f32) / 16.0,
        Resolution::Bits11 => (raw_temp as f32) / 16.0,
        Resolution::Bits10 => (raw_temp as f32) / 16.0,
        Resolution::Bits9 => (raw_temp as f32) / 16.0,
    };
    Ok(SensorData {
        temperature,
        resolution,
        alarm_temp_high: i8::from_le_bytes([scratchpad[2]]),
        alarm_temp_low: i8::from_le_bytes([scratchpad[3]]),
    })
  }  

  pub fn get_temperature(&mut self) -> anyhow::Result<f64> {

    // initiate a temperature measurement for all connected devices
    ds18b20::start_simultaneous_temp_measurement(&mut self.bus, &mut self.delay).into_anyhow()?;

    // wait until the measurement is done. This depends on the resolution you specified
    // If you don't know the resolution, you can obtain it from reading the sensor data,
    // or just wait the longest time, which is the 12-bit resolution (750ms)
    self.resolution.delay_for_measurement_time(&mut self.delay);

    // let sensor_data = self.sensor
    //   .read_data(&mut self.bus, &mut self.delay).into_anyhow()?;
    let sensor_data = self.read_data()?;
    info!("Device at {:?} is {}°C (res {:?})", 
      self.sensor.address(), sensor_data.temperature, sensor_data.resolution
    );


    Ok(sensor_data.temperature as f64)
  }
}

