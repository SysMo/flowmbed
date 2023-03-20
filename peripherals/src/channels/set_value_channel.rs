use serde::Serialize;
use serde::de::DeserializeOwned;
use super::messages::SetValueAction;
use super::channel_bus::{ChannelBus, ForwardChannel, ReverseChannel, IOConnector};
use crate::util::QualifiedPath;

pub struct SetValueChannel<V: Clone> {
  id: String,
  pub reader: ForwardChannel<V>,
  pub action: ReverseChannel<SetValueAction<V>>
}

impl<V: Clone + Serialize + DeserializeOwned> SetValueChannel<V> {
  pub fn new(id: &str) -> Self {
    SetValueChannel {
      id: id.to_owned(),
      reader: ForwardChannel::new("reader"),
      action: ReverseChannel::new("action"),
    }
  }

  pub fn handle_actions<F>(&self, mut f: F) where F: FnMut(SetValueAction<V>, &ForwardChannel<V>) {
    match &self.action.receiver {
      Some(receiver) => {
        while let Some(v) = receiver.next() {
          f(v, &self.reader);
        }     
      },
      None => (),
  }
  }
}

impl<V: Clone + Serialize + DeserializeOwned> IOConnector for SetValueChannel<V> {
  fn connect_io(&mut self, comm: &mut dyn crate::mqtt::MqttService, qpath: &QualifiedPath) {
      self.reader.connect_io(comm, &qpath.append(&self.id));
      self.action.connect_io(comm, &qpath.append(&self.id));
  }
}