use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::util::QualifiedPath;
use crate::mqtt::{MqttService, MessageSender, MessageReceiver};

// pub enum StreamChannelDirection {
//   Forward, Reverse, BiDirectional
// }


pub trait ChannelBus {
  fn get_id(&self) -> &str;
}


pub trait IOConnector {
  fn connect_io(&mut self, comm: &mut dyn MqttService, qpath: &QualifiedPath);
  fn connect_io_root(&mut self, comm: &mut dyn MqttService) {
    self.connect_io(comm, QualifiedPath::empty_ref());
  }
}

pub trait DeviceBusConnector<D>   {
  fn sample(&self, device: &D);
  fn handle_actions(&self, device: &mut D);
}

pub struct ForwardChannel<V: Clone> {
  pub id: String,
  pub publisher: Option<Box<dyn MessageSender<V>>>,
}

impl<V: Clone + Serialize> ForwardChannel<V> {
  pub fn new(id: &str) -> Self {
    ForwardChannel { id: id.to_owned(), publisher: None }
  }

  pub fn send(&self, v: V) -> anyhow::Result<()> {
    match &self.publisher {
        Some(publisher) => publisher.send(v),
        None => anyhow::bail!("no publisher!"),
    }
  }
}

impl<V: Clone + Serialize> IOConnector for ForwardChannel<V> {
  fn connect_io(&mut self, comm: &mut dyn crate::mqtt::MqttService, qpath: &QualifiedPath) {
      let topic = qpath.append(&self.id).join("/");
      self.publisher = Some(Box::new(comm.publisher(&topic)));
  }
}

pub struct ReverseChannel<V: Clone> {
  pub id: String,
  pub receiver: Option<Box<dyn MessageReceiver<V>>>,
}

impl<V: Clone + DeserializeOwned> ReverseChannel<V> {
  pub fn new(id: &str) -> Self {
    ReverseChannel { id: id.to_owned(), receiver: None }
  }

  pub fn handle_actions<F>(&self, mut f: F) where F: FnMut(V) {
    match &self.receiver {
        Some(receiver) => {
          while let Some(v) = receiver.next() {
            f(v);
          }     
        },
        None => (),
    }
  }
}

impl<V: Clone + DeserializeOwned> IOConnector for ReverseChannel<V> {
  fn connect_io(&mut self, comm: &mut dyn crate::mqtt::MqttService, qpath: &QualifiedPath) {
      let topic = qpath.append(&self.id).join("/");
      self.receiver = Some(Box::new(comm.subscriber(&topic)));
  }
}
