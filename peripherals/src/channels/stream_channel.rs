// use super::channel_bus::{MessageSender, MessageReceiver};

// pub struct ForwardChannel<V: Clone> {
//   pub id: String,
//   pub publisher: Option<Box<dyn MessageSender<V>>>,
// }

// impl<V: Clone> ForwardChannel<V> {
//   pub fn new(id: &str) -> Self {
//     ForwardChannel { id: id.to_owned(), publisher: None }
//   }

  
// }

// pub struct ReverseChannel<V: Clone> {
//   pub id: String,
//   pub receiver: Option<Box<dyn MessageReceiver<V>>>,
// }

// impl<V: Clone> ReverseChannel<V> {
//   pub fn new(id: &str) -> Self {
//     ReverseChannel { id: id.to_owned(), receiver: None }
//   }
// }


