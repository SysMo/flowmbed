use embedded_svc::mqtt::client::{Client, Connection, MessageImpl, Publish, QoS, Event, Message};
use embedded_svc::utils::mqtt::client::{ConnState};
use esp_idf_svc::mqtt::client::*;
use esp_idf_sys::EspError;
use anyhow::Result;
use log::*;
use std::str::{from_utf8, from_utf8_unchecked};
use std::thread;
use super::mqtt_handler::{MQTTHandler, HandlerResult, Inbox};

pub struct Config {
  client_id: &'static str,
  group_id: &'static str,
  url: &'static str,
  user: &'static str,
  password: &'static str,
}

static config: Config = Config {
  client_id: "node1",
  group_id: "esp-reform",
  url: "mqtt://192.168.68.111:1883",
  user: "guest",
  password: "guest",
};

pub fn start(inbox: &Inbox) -> Result<EspMqttClient<ConnState<MessageImpl, EspError>>> {
  info!("About to start MQTT client");

  let conf = MqttClientConfiguration {
      client_id: Some(config.client_id),
      crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
      username: Some(config.user),
      password: Some(config.password),
      ..Default::default()
  };

  let (mut client, mut connection) =
      EspMqttClient::new_with_conn(config.url, &conf)?;

  info!("MQTT client started");

  let sender = inbox.clone();
  let in_topic = format!("{}-{}", config.group_id, config.client_id);
  let out_topic = format!("{}-central", config.group_id);

  // Need to immediately start pumping the connection for messages, or else subscribe() and publish() below will not work
  // Note that when using the alternative constructor - `EspMqttClient::new` - you don't need to
  // spawn a new thread, as the messages will be pumped with a backpressure into the callback you provide.
  // Yet, you still need to efficiently process each message in the callback without blocking for too long.
  //
  // Note also that if you go to http://tools.emqx.io/ and then connect and send a message to topic
  // "rust-esp32-std-demo", the client configured here should receive it.
  thread::spawn(move || {
      info!("MQTT Listening for messages");
      
      let mut listener_active = true;
      let mut next_delay: i64 = 1000;



      while listener_active {
        while let Some(event) = connection.next() {
          match event {
              Err(e) => info!("MQTT Message ERROR: {}", e),
              Ok(event) => match event {
                Event::Received(msg) => {
                  let msg_str = {
                    String::from_utf8(msg.data().to_vec()).unwrap()
                  };
                  sender.send(msg);
                  ()
                }
                // match handler.process_msg(msg) {
                //   HandlerResult::Continue(delay) => {
                //     next_delay = delay;
                //   }
                //   HandlerResult::Break(reason) => {
                //     info!("{}", reason);
                //     listener_active = false;
                //   }
                // }
                Event::BeforeConnect => (),
                Event::Connected(_) => info!("MQTT Connected"),
                _ => warn!("Cannot process event {:?}", event)
              }
            }
          }
      }


      info!("MQTT connection loop exit");
  });

  client.subscribe(&in_topic, QoS::AtMostOnce)?;

  info!("Subscribed to '{}'", in_topic);

  client.publish(
      &out_topic,
      QoS::AtMostOnce,
      false,
      "Hello from esp-reform".as_bytes(),
  )?;

  info!("Published a hello message to topic '{}'", out_topic);

  Ok(client)
}