use embedded_hal_0_2::blocking::delay::{DelayUs, DelayMs};
use embedded_hal_0_2::digital::v2::{InputPin, OutputPin};
use ds18b20::{Ds18b20, read_scratchpad, SensorData};
use one_wire_bus::{OneWire, OneWireResult, OneWireError};
use log::*;
use core::fmt::{Debug, Display};
pub use ds18b20::Resolution;
use flowmbed_dynsys::core::Float;

use super::traits::AnalogReaderMultiChannel;
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

pub struct DS18B20Array<P, E, D, const N: usize>
where P: OutputPin<Error = E> + InputPin<Error = E>, 
      E: Debug + Display + std::marker::Sync,
      D: DelayUs<u16> + DelayMs<u16>
{
  bus: OneWire<P>,
  delay: D,
  sensors: [Option<Ds18b20>; N],
  resolution: Resolution,
  alarm_high: i8,
  alarm_low: i8,
}

impl<P, E, D, const N: usize> DS18B20Array<P, E, D, N> 
where P: OutputPin<Error = E> + InputPin<Error = E>, 
      E: Debug + Display + std::marker::Sync,
      D: DelayUs<u16> + DelayMs<u16>
{
  pub fn new(pin: P, delay: D) -> anyhow::Result<Self> {
    const SENSOR_NONE: Option<Ds18b20> = None;

    let mut device_array = DS18B20Array {
      bus: OneWire::new(pin).into_anyhow()?, 
      delay,
      sensors: [SENSOR_NONE; N],
      resolution: Resolution::Bits12, 
      alarm_high: 100, alarm_low: 0
    };
    device_array.find_devices()?;
    device_array.write_settings()?;
    Ok(device_array)
  }

  fn find_devices(&mut self) -> anyhow::Result<()> {
    let mut num_found = 0;
   
    for device_address in self.bus.devices(false, &mut self.delay) {
      // The search could fail at any time, so check each result. The iterator automatically
      // ends after an error.
      let device_address = device_address.into_anyhow()?;

      if device_address.family_code() == ds18b20::FAMILY_CODE {
        // The family code can be used to identify the type of device
        // If supported, another crate can be used to interact with that device at the given address
        info!("Found device at address {:?} with family code: {:#x?}",
              device_address, device_address.family_code());
      } else {
        // skip to other devices
        continue;
      }

      match Ds18b20::new::<anyhow::Error>(device_address) {
        Ok(x) => {
          if num_found > N - 1 {
            anyhow::bail!("Two many devices found, expected {}", N);
          }
          self.sensors[num_found] = Some(x);
          num_found += 1;
        }
        Err(x) => {
          warn!("{:?}", x)
        }
      }
    }
    
    info!("Found {num_found} devices");

    if num_found < N {
      anyhow::bail!("Not enough devices found, expected {}, found {}", N, num_found);
    }


    return Ok(())
  }



  pub fn set_resolution(&mut self, r: Resolution) -> anyhow::Result<()> {
    self.resolution = r;
    self.write_settings()
  }

  pub fn write_settings(&mut self) -> anyhow::Result<()> {
    for sensor in &mut self.sensors {
      match sensor{
        Some(s) => s.set_config(self.alarm_low, self.alarm_high, self.resolution, 
          &mut self.bus, &mut self.delay
        ).into_anyhow()?,
        None => ()
      }  
    }
    Ok(())
  }

  fn read_data(&mut self, sensor_index: usize) -> anyhow::Result<SensorData> {    
    let sensor = match &mut self.sensors[sensor_index] {
        Some(s) => s,
        None => anyhow::bail!("sensor not initialized!"),
    };

    let scratchpad = read_scratchpad(
      sensor.address(), &mut self.bus, &mut self.delay
    ).into_anyhow()?;

    let resolution = match scratchpad[4] {
      0b00011111 => Resolution::Bits9,
      0b00111111 => Resolution::Bits10,
      0b01011111 => Resolution::Bits11,
      0b01111111 => Resolution::Bits12,
      _ => return Err(OneWireError::<E>::CrcMismatch).into_anyhow(),
    };

    let raw_temp = u16::from_le_bytes([scratchpad[0], scratchpad[1]]);
    const F_1_16: f32 = 1.0 / 16.0;
    let temperature = if raw_temp > 0x7ff {
      -(((raw_temp & 0x7ff) ^ 0x7ff) as f32) * F_1_16
    } else {
      ((raw_temp & 0x7ff) as f32) * F_1_16
    };


    // let temperature = match resolution {
    //     Resolution::Bits12 => (raw_temp as f32) / 16.0,
    //     Resolution::Bits11 => (raw_temp as f32) / 16.0,
    //     Resolution::Bits10 => (raw_temp as f32) / 16.0,
    //     Resolution::Bits9 => (raw_temp as f32) / 16.0,
    // };
    Ok(SensorData {
        temperature,
        resolution,
        alarm_temp_high: i8::from_le_bytes([scratchpad[2]]),
        alarm_temp_low: i8::from_le_bytes([scratchpad[3]]),
    })
  }  

}

impl<P, E, D, const N: usize> AnalogReaderMultiChannel<N> for DS18B20Array<P, E, D, N>
where P: OutputPin<Error = E> + InputPin<Error = E>, 
      E: Debug + Display + std::marker::Sync,
      D: DelayUs<u16> + DelayMs<u16>
{
  fn read_channel(&mut self, id: usize) -> anyhow::Result<Float> {
    let sensor = self.sensors[id].as_mut()
      .ok_or(anyhow::anyhow!("Channel {id} not active"))?;
    sensor.start_temp_measurement(
      &mut self.bus, &mut self.delay
    ).into_anyhow()?;

    self.resolution.delay_for_measurement_time(&mut self.delay);

    self.read_data(id).map(|x| x.temperature)
  }

  fn read_all(&mut self) -> anyhow::Result<[Float; N]> {
    // initiate a temperature measurement for all connected devices
    ds18b20::start_simultaneous_temp_measurement(&mut self.bus, &mut self.delay).into_anyhow()?;

    // wait until the measurement is done. This depends on the resolution you specified
    // If you don't know the resolution, you can obtain it from reading the sensor data,
    // or just wait the longest time, which is the 12-bit resolution (750ms)
    self.resolution.delay_for_measurement_time(&mut self.delay);

    let mut readings = [0.0; N];

    for i in 0..self.sensors.len() {
      match self.read_data(i)  {
        Ok(data) => {
          readings[i] = data.temperature;
        },
        Err(_) => anyhow::bail!("Failed reading channel {i}")
      }
    }

    Ok(readings)
  }
}