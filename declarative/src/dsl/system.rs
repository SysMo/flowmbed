use serde::{Serialize, Deserialize};
use super::event_loop::EventLoopConfigEnum;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemConfig {
  id: String,
  event_loop: EventLoopConfigEnum,
  sources: Vec<String>
}

