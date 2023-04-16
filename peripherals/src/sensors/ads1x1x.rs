use std::sync::Arc;
use std::sync::Mutex;

use super::traits::{AnalogReaderMultiChannel, AnalogReader};
use ads1x1x::interface::I2cInterface;
use ads1x1x::{channel, Ads1x1x, SlaveAddr};
pub use ads1x1x::{FullScaleRange, ChannelSelection};
use embedded_hal_0_2::blocking;
use embedded_hal_0_2::adc::OneShot;
use nb::block;
use serde::__private::de;

pub struct Ads1x1xDeviceConfigurator<I2C, const N: usize> {
  i2c: I2C, 
  range: FullScaleRange,
  channels: [ChannelSelection; N]
}

impl<I2C, E: std::fmt::Debug> Ads1x1xDeviceConfigurator<I2C, 0>
where I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E> {
  pub fn new(i2c: I2C, range: FullScaleRange) -> Self {
    Self {
      i2c, range, channels: []
    }
  }
}

impl<I2C, E: std::fmt::Debug, const N: usize> Ads1x1xDeviceConfigurator<I2C, N>
where I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E> {
  pub fn add_channel(self, channel: ChannelSelection) -> Ads1x1xDeviceConfigurator<I2C, { N + 1 }> {
    let mut channels = [ChannelSelection::SingleA0; {N + 1}];
    for i in 0..N {
      channels[i] = self.channels[i]
    }
    channels[N] = channel;

    Ads1x1xDeviceConfigurator::<I2C, { N + 1 }> {
        i2c: self.i2c,
        range: self.range,
        channels: channels,
    }
  }

  pub fn build(self) -> Ads1115Reader<I2C, N> {
    let address = SlaveAddr::default();
    let mut device = Ads1x1x::new_ads1115(self.i2c, address);
    device.set_full_scale_range(self.range.clone()).unwrap();
    Ads1115Reader { 
      range: self.range, device, channels: self.channels
    }
  }
}

pub struct Ads1115Reader<I2C, const N: usize> {
  range: FullScaleRange,
  device: Ads1x1x<
    I2cInterface<I2C>,
    ads1x1x::ic::Ads1115, 
    ads1x1x::ic::Resolution16Bit,
    ads1x1x::mode::OneShot
  >,
  channels: [ChannelSelection; N]
}

impl <I2C, E: std::fmt::Debug, const N: usize> Ads1115Reader<I2C, N> 
where I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E> {
  fn read_channel_raw(&mut self, channel: ChannelSelection) -> anyhow::Result<f32> {
    let read_result = match channel {
        ChannelSelection::SingleA0 => 
          block!(self.device.read(&mut channel::SingleA0)),
        ChannelSelection::SingleA1 => 
          block!(self.device.read(&mut channel::SingleA1)),
        ChannelSelection::SingleA2 => 
          block!(self.device.read(&mut channel::SingleA2)),
        ChannelSelection::SingleA3 => 
          block!(self.device.read(&mut channel::SingleA3)),
        ChannelSelection::DifferentialA0A1 => 
          block!(self.device.read(&mut channel::DifferentialA0A1)),
        ChannelSelection::DifferentialA0A3 => 
          block!(self.device.read(&mut channel::DifferentialA0A3)),
        ChannelSelection::DifferentialA1A3 => 
          block!(self.device.read(&mut channel::DifferentialA1A3)),
        ChannelSelection::DifferentialA2A3 => 
          block!(self.device.read(&mut channel::DifferentialA2A3)),
    };

    let v_max = match self.range {
      FullScaleRange::Within6_144V => 6.144,
      FullScaleRange::Within4_096V => 4.096,
      FullScaleRange::Within2_048V => 2.048,
      FullScaleRange::Within1_024V => 1.024,
      FullScaleRange::Within0_512V => 0.512,
      FullScaleRange::Within0_256V => 0.256,
    };
    let mult: f32 = v_max / 32768.0; 
    let res = read_result.map(|x| ( x as f32) * mult);
    match res {
      Ok(v) => Ok(v),
      Err(e) => Err(anyhow::anyhow!("{:?}", e)),
    }
  }

  pub fn split(self) -> [Ads1115ReaderChannel<I2C, N>; N] {
    let channels = self.channels.clone();
    let reader = Arc::new(Mutex::new(self));
    let reader_channels: [Ads1115ReaderChannel<I2C, N>; N] = channels.map(|ch|
      Ads1115ReaderChannel::<I2C, N> {
        reader: reader.clone(),
        channel: ch,
      }
    );

    reader_channels
  }
}

impl<I2C, E: std::fmt::Debug, const N: usize> AnalogReaderMultiChannel<N> for Ads1115Reader<I2C, N> 
where I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E> {
  fn read_all(&mut self) -> anyhow::Result<[f32; N]> {
    let mut readings = [0.0; N];
    for i in 0..N {
      match self.read_channel_raw(self.channels[i]) {
        Ok(v) => readings[i] = v,
        Err(err) => return Err(anyhow::anyhow!(err)),
      }
    }

    Ok(readings)
  }

  fn read_channel(&mut self, id: usize) -> anyhow::Result<f32> {
      if id < N {
        // convert_diff_voltage(self.device.read(&mut channel::DifferentialA0A1), self.range)
        self.read_channel_raw(self.channels[id])
      } else {
        anyhow::bail!("Incorrect channel id {}", id)
      }
  }
}

#[derive(Clone)]
pub struct Ads1115ReaderChannel<I2C, const N: usize> {
  reader: Arc<Mutex<Ads1115Reader<I2C, N>>>,
  channel: ChannelSelection,
}

impl<I2C, E: std::fmt::Debug, const N: usize> AnalogReader for Ads1115ReaderChannel<I2C, N> 
where I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E> {
  fn read(&mut self) -> anyhow::Result<f32> {
      let mut reader = self.reader.lock().unwrap();
      reader.read_channel_raw(self.channel)
  }
}
