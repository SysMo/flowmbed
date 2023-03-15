use embedded_svc::mqtt::client::{self, Client, Connection, MessageImpl, Publish, QoS, Event, Message};
use embedded_svc::utils::mqtt::client::{ConnState};
use esp_idf_svc::mqtt::client::{MqttClientConfiguration, EspMqttClient, EspMqttMessage};
use esp_idf_sys::EspError;
use anyhow::Result;
use log::*;
use serde::{Deserialize, Serialize};
use std::str::{from_utf8, from_utf8_unchecked};
use std::thread;
// use super::mqtt_handler::{MQTTHandler, HandlerResult, Inbox};

pub struct Config {
  group_id: &'static str,
  client_id: &'static str,
  url: &'static str,
  user: &'static str,
  password: &'static str,
}

// static config: Config = Config {
//   client_id: "sysmo-esp32-node1",
//   url: "mqtt://public.mqtthq.com",
//   user: "sysmo",
//   password: "sysmopass",
// };

static config: Config = Config {
  group_id: "sysmo/esp32",
  client_id: "bio1",
  url: "mqtts://c0a7164f.ala.us-east-1.emqxsl.com",
  user: "sysmo",
  password: "sysmopass",
};




pub struct MqttService {
  client: EspMqttClient<()>,
  out_topic: String,
}

fn callback<'b>(msg: &'b Result<client::Event<EspMqttMessage<'b>>, EspError>) {
  println!("{msg:?}");
}

impl MqttService {
  pub fn new() -> anyhow::Result<MqttService> {
    info!("About to start MQTT client");
  
    let conf = MqttClientConfiguration {
        client_id: Some(config.client_id),
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        username: Some(config.user),
        password: Some(config.password),
        ..Default::default()
    };
  
    // let (mut client, mut connection) =
    //     EspMqttClient::new_with_conn(config.url, &conf)?;
    let mut client = EspMqttClient::new(config.url, &conf, &callback)?;
  
    let in_topic = format!("{}/{}/to", config.group_id, config.client_id);
    let out_topic = format!("{}/{}/from", config.group_id, config.client_id);
    info!("MQTT client started");
  
    client.subscribe(&in_topic, QoS::AtMostOnce)?;
    
    // client.publish(
    //     &out_topic,
    //     QoS::AtMostOnce,
    //     false,
    //     "Hello from esp-reform".as_bytes(),
    // )?;
  
    info!("Published a hello message to topic '{}'", out_topic);
  
    Ok(MqttService { client, out_topic })
  }


  pub fn publish_str(&mut self, s: &str) -> anyhow::Result<()> {
    self.client.publish(
      &self.out_topic,
      QoS::AtMostOnce,
      false,
      s.as_bytes()
    )?;
    Ok(())
  }

  pub fn publish_obj<T: Serialize>(&mut self, obj: &T) -> anyhow::Result<()> {
    self.client.publish(
      &self.out_topic,
      QoS::AtMostOnce,
      false,
      serde_json::to_string(&obj)?.as_bytes()
    )?;
    Ok(())
  }
}
