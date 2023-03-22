use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait MessageSender<V: Serialize> {
  fn send(&self, payload: V) -> anyhow::Result<()>;
}

pub trait MessageReceiver<V: DeserializeOwned> {
  fn next(&self) -> Option<V>;
}
