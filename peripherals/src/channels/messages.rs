use serde::{Serialize, Deserialize, de::DeserializeOwned};
use time::OffsetDateTime;

pub trait MeasurementValueTrait: Clone + std::fmt::Debug + Serialize + DeserializeOwned {}
impl<V> MeasurementValueTrait for V where V: Clone + std::fmt::Debug + Serialize + DeserializeOwned {}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Measurement<V> {
  #[serde(with = "time::serde::iso8601")]
  pub timestamp: OffsetDateTime,
  pub value: V
}

impl<V: MeasurementValueTrait> Measurement<V> {
  pub fn new(value: V) -> Self {
    let timestamp = OffsetDateTime::now_utc();
    log::info!("Created timestamp {}", timestamp);
    Self { 
      timestamp: timestamp, 
      value 
    }
  }
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "runtimeType")]
#[serde(rename_all = "camelCase")]
pub enum SetValueAction<V> {
  SetValue {value: V},
  ReadValue
}

#[macro_export]
macro_rules! set_value_handler {
    ($bus_ident: ident, $( $bus_field:ident ).+, $device_ident: ident, $device_field: ident) => {
      $bus_ident.$($bus_field).+.handle_actions(|action, sender| {
        match action {
          flowmbed_peripherals::channels::SetValueAction::SetValue { value: v } => {
            $device_ident.$device_field = v;
            sender($device_ident.$device_field)?;
            log::info!("Setting {} to {}", stringify!($device_field), v);
          },
          flowmbed_peripherals::channels::SetValueAction::ReadValue => {
            sender($device_ident.$device_field)?;
          },
        }
        Ok(())
      });
    };
}


