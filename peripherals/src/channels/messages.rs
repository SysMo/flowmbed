use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Measurement<V: Clone + Serialize> {
  pub timestamp: String,
  pub value: V
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


