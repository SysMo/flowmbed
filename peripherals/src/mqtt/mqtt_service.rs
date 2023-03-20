use log::*;
use serde::Serialize;
use serde::de::DeserializeOwned;

use std::sync::mpsc;
use super::{MessageReceiver, MessageSender};

pub struct MqttServiceOptions {
  pub host: String, 
  pub client_id: String, 
  pub user: String, 
  pub password: String
}

pub struct StrMessage {
  pub topic: String,
  pub payload: String,
}

impl StrMessage {
  pub fn new(topic: String, payload: String,) -> Self {
    Self { topic, payload }
  }
}

pub struct MqttPublisher {
  pub topic: String,
  pub publisher_tx: mpsc::Sender<StrMessage>,
}

impl<T: Serialize> MessageSender<T> for MqttPublisher {
  fn send(&self, payload: T) -> anyhow::Result<()> {
    match serde_json::to_string(&payload) {
      Ok(payload_str) => {
        let msg = StrMessage {
          topic: self.topic.clone(), 
          payload: payload_str
        };
        self.publisher_tx.send(msg)?;
        Ok(())
      },
      Err(e) => {
        error!("Failed serializing message!");
        error!("Reason: {}", e);
        Err(anyhow::anyhow!(e))
      },
    }
  }
}

pub struct MqttSubscriber {
  pub subscriber_rx: mpsc::Receiver<String>
}

impl<T: DeserializeOwned> MessageReceiver<T> for MqttSubscriber {
  fn next(&self) -> Option<T>  {
    match self.subscriber_rx.try_recv() {
        Ok(msg_str) => {
          let msd_deser = serde_json::from_str(&msg_str);
          match msd_deser {
            Ok(v) => Some(v),
            Err(e) => {
              error!("Failed deserializing message: {msg_str}");
              error!("Reason: {}", e);
              None
            },
          }
        },
        Err(_) => None,
    }
  }
}

pub trait MqttService {
  fn publisher(&self, topic: &str) -> MqttPublisher;
  fn subscriber(&mut self, topic: &str) -> MqttSubscriber;
}

