use serde::Serialize;
use super::channel_bus::{ForwardChannel, IOConnector};
use crate::util::QualifiedPath;
use super::messages::Measurement;

pub struct MeasurementChannel<V: Clone + Serialize> {
  pub channel: ForwardChannel<Measurement<V>>
}

impl<V: Clone + Serialize> MeasurementChannel<V> {
  pub fn new(id: &str) -> Self {
    MeasurementChannel { 
      channel: ForwardChannel::new(id) 
    }
  }

  pub fn sample(&self, v: V) {
    match self.channel.send(Measurement {
      timestamp: "now".to_owned(),
      value: v
    }) {
        Ok(_) => (),
        Err(_) => {
          log::warn!("failed sending value");
        },
    }
  }
}

impl<V: Clone + Serialize> IOConnector for MeasurementChannel<V> {
  fn connect_io(&mut self, comm: &mut dyn crate::mqtt::MqttService, qpath: &QualifiedPath) {
    self.channel.connect_io(comm, qpath);
  }
}