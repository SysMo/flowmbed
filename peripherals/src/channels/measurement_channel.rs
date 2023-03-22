use super::channel_bus::{ForwardChannel, IOConnector};
use crate::util::QualifiedPath;
use super::messages::{Measurement, MeasurementValueTrait};

pub struct MeasurementChannel<V: MeasurementValueTrait> {
  pub channel: ForwardChannel<Measurement<V>>
}

impl<V: MeasurementValueTrait> MeasurementChannel<V> {
  pub fn new(id: &str) -> Self {
    MeasurementChannel { 
      channel: ForwardChannel::new(id) 
    }
  }

  pub fn sample(&self, v: V) {
    match self.channel.send(Measurement::new(v)) {
        Ok(_) => (),
        Err(_) => {
          log::warn!("failed sending value");
        },
    }
  }
}

impl<V: MeasurementValueTrait> IOConnector for MeasurementChannel<V> {
  fn connect_io(&mut self, comm: &mut dyn crate::mqtt::MqttService, qpath: &QualifiedPath) {
    self.channel.connect_io(comm, qpath);
  }
}