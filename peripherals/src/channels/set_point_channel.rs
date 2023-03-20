use super::{MeasurementChannel, SetValueChannel};
use super::messages::SetValueAction;
use super::channel_bus::{ForwardChannel, ReverseChannel, ChannelBus, IOConnector};
use crate::util::QualifiedPath;
use crate::mqtt::MqttService;

pub struct SetPointChannel {
  id: String,
  pub current: MeasurementChannel<f32>,
  pub target: SetValueChannel<f32>
}

impl SetPointChannel {
  pub fn new(id: &str) -> Self {
    SetPointChannel {
      id: id.to_owned(),
      current: MeasurementChannel::new("current"),
      target: SetValueChannel::new("target"),
    }
  }
}

impl IOConnector for SetPointChannel {
  fn connect_io(&mut self, comm: &mut dyn MqttService, qpath: &QualifiedPath) {
      self.current.connect_io(comm, &qpath.append(&self.id));
      self.target.connect_io(comm, &qpath.append(&self.id));
  }
}