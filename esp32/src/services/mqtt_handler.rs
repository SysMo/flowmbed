use log::*;
use embedded_svc::mqtt::client::{Message, MessageImpl};
use std::cell::RefCell;
use std::str::{from_utf8, Utf8Error};
use std::sync::mpsc;
pub enum HandlerResult {
  Continue(i64),
  Break(String)
}

pub type Inbox = mpsc::SyncSender<MessageImpl>;
pub type Outbox = mpsc::Receiver<MessageImpl>;
pub trait MQTTHandler {
  fn inbox(&self) -> &Inbox;
  fn check_inbox(&self) -> Option<String>;
}

pub struct SimpleHandler {
  msg_in_tx: Inbox,
  msg_in_rx: mpsc::Receiver<MessageImpl>,
}

impl SimpleHandler {
  pub fn new() -> SimpleHandler {
    let (msg_in_tx, msg_in_rx) = 
      mpsc::sync_channel(1);
    SimpleHandler {
      msg_in_tx: msg_in_tx,
      msg_in_rx: msg_in_rx,
    }
  }  
}

impl MQTTHandler for SimpleHandler {
  fn inbox(&self) -> &Inbox {
    &self.msg_in_tx
  }

  fn check_inbox(&self) -> Option<String> {
    match self.msg_in_rx.recv() {
      Ok(msg) => match from_utf8(msg.data()) {
        Ok(x) => Some(x.to_owned()),
        Err(err) => {
          error!("Cannot convert message {} to string", msg.id());
          None
        }
      },
      Err(err) => None
    }
  }

}