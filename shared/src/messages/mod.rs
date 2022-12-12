use serde::{Serialize, Deserialize};

pub mod command;
pub mod query;
pub mod dynsys;

#[derive(Serialize, Deserialize, Debug)]
pub enum SystemMessage {
  Command(command::Command),
  CommandSeq(Vec<command::Command>),
  Query(u128, query::Query)
}

impl SystemMessage {
  pub fn test_round_trip(msg: SystemMessage)
  // where T: for<'de> Deserialize<'de>  + std::fmt::Debug 
  {

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&msg).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);
    
    // Convert the JSON string back to a Point.
    let deserialized: SystemMessage = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

  }

  pub fn query(q: query::Query) -> SystemMessage {
    SystemMessage::Query(
      query::gen_id(),
      q
    )
  }

  pub fn command(cmd: command::Command) -> SystemMessage {
    SystemMessage::Command(cmd)
  }

}


// use serde::{Serialize, Deserialize};

// pub mod MessageBuilder {
//   use super::*;

//   pub mod command {
    
//   }
//   pub mod query {
//     use super::*;
//     use uuid::Uuid;


//     pub mod param {
//       use super::*;
//       pub fn real(param_id: u32) -> SystemMessage {
//         let uid = gen_id();
//         SystemMessage::Query(uid, Query::GetRealParameter(param_id))
//       }
//     }
//   }
// }