use serde::Serialize;
use super::channel_bus::{ChannelBus, ForwardChannel, IOConnector};
use crate::util::QualifiedPath;

pub struct MeasurementChannel<V: Clone + Serialize> {
  pub channel: ForwardChannel<V>
}

impl<V: Clone + Serialize> MeasurementChannel<V> {
  pub fn new(id: &str) -> Self {
    MeasurementChannel { 
      channel: ForwardChannel::new(id) 
    }
  }

  pub fn sample(&self, v: V) -> anyhow::Result<()>  {
    self.channel.send(v)
  }
}

impl<V: Clone + Serialize> IOConnector for MeasurementChannel<V> {
  fn connect_io(&mut self, comm: &mut dyn crate::mqtt::MqttService, qpath: &QualifiedPath) {
    self.channel.connect_io(comm, qpath);
  }
}