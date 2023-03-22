use flowmbed_peripherals::mqtt::{MqttServiceOptions};
use db_collector::{InfluxDbOptions, InfluxDbAgent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

  simple_logger::init_with_level(log::Level::Info).unwrap();

  let influxdb_options = InfluxDbOptions {
    host: "http://localhost:8086".to_owned(),
    bucket: "bio-test1".to_owned(),
    token: "btCOTp0cn0rolkH-8VgU34V7_EliMEqVmP88UhoTiNS1pJEOX9Hmsl1BIk5wjRVxXQ6_6Agb7SXR2p75PC82CQ==".to_owned(),
    measurement: "bioreactor".to_owned(),
  };

  
  let mqtt_options = MqttServiceOptions {
    host: "mqtts://c0a7164f.ala.us-east-1.emqxsl.com:8883".to_owned(),
    client_id: "influxdb_conn".to_owned(),
    user: "sysmo".to_owned(), 
    password: "sysmopass".to_owned(),
  };

  let db_agent = InfluxDbAgent::new(
    influxdb_options, mqtt_options, "bioreactor/#"
  );

  db_agent.start().await?;

  Ok(())
}


