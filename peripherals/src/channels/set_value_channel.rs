use super::messages::SetValueAction;
use super::channel_bus::{ForwardChannel, ReverseChannel, IOConnector};
use crate::util::QualifiedPath;
use super::messages::{Measurement, MeasurementValueTrait};

pub struct SetValueChannel<V: MeasurementValueTrait> {
  id: String,
  pub reader: ForwardChannel<Measurement<V>>,
  pub action: ReverseChannel<SetValueAction<V>>
}

// trait ActionHandler<V> : FnMut(SetValueAction<V>, &dyn FnOnce(V) -> anyhow::Result<()>) {}

impl<V: MeasurementValueTrait> SetValueChannel<V> {
  pub fn new(id: &str) -> Self {
    SetValueChannel {
      id: id.to_owned(),
      reader: ForwardChannel::new("reader"),
      action: ReverseChannel::new("action"),
    }
  }

  // pub fn send_current(&self, v: V) -> anyhow::Result<()> {
  //   self.reader.send(Measurement {
  //     timestamp: "now".to_owned(),
  //     value: v
  //   })
  // }

  pub fn handle_actions<F>(&self, mut f: F) 
  where F: FnMut(SetValueAction<V>, &dyn Fn(V) -> anyhow::Result<()>) -> anyhow::Result<()> {
    let send_current = |v: V| self.reader.send(Measurement::new(v));

    match &self.action.receiver {
      Some(receiver) => {
        while let Some(v) = receiver.next() {
          match f(v, &send_current) {
            Ok(_) => (),
            Err(_) => {
              log::warn!("failed sending data")
            },
        };
        }     
      },
      None => (),
  }
  }
}

impl<V: MeasurementValueTrait> IOConnector for SetValueChannel<V> {
  fn connect_io(&mut self, comm: &mut dyn crate::mqtt::MqttService, qpath: &QualifiedPath) {
      self.reader.connect_io(comm, &qpath.append(&self.id));
      self.action.connect_io(comm, &qpath.append(&self.id));
  }
}