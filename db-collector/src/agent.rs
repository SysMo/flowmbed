use flowmbed_peripherals::mqtt::mqtt_service::StrMessage;
use influxdb::{Query, Timestamp};
use flowmbed_peripherals::mqtt::{MqttAsyncServiceImpl, MqttServiceOptions};
use flowmbed_peripherals::channels::messages::{Measurement, MeasurementValueTrait};
// use chrono::{DateTime, Utc};
use time::{OffsetDateTime, Time};

pub struct InfluxDbOptions {
  pub host: String,
  pub bucket: String,
  pub token: String,
  pub measurement: String
}

pub struct MeasurementPoint<'a, V: MeasurementValueTrait> {
  pub timestamp: OffsetDateTime,
  pub measurement: &'a str,
  pub sensor: &'a str,
  pub value: V
}

pub struct InfluxDbAgent {
  db_options: InfluxDbOptions, 
  mqtt_options: MqttServiceOptions,
  mqtt_topic: String,
}

impl InfluxDbAgent {
  pub fn new(db_options: InfluxDbOptions, mqtt_options: MqttServiceOptions, mqtt_topic: &str) -> Self{

    Self {
      db_options, 
      mqtt_options, 
      mqtt_topic: mqtt_topic.to_owned(),
    }
  }

  pub async fn start(&self) -> anyhow::Result<()> {
    let db_client = influxdb::Client::new(
      &self.db_options.host, 
      &self.db_options.bucket
    ).with_token(&self.db_options.token);

    let mqtt_client = MqttAsyncServiceImpl::new(
      &self.mqtt_options, &self.mqtt_topic
    ).await?;

    use futures_util::StreamExt;
    let mut msg_stream = mqtt_client.stream;

    log::info!("Starting event loop");
    while let Some(item) = msg_stream.next().await {    
      match item {
        Some(msg) => {
          if let Ok(point) = self.try_parse_measurement::<f64>(&msg) {
            Self::save_measurement(&db_client, point).await
          } else if let Ok(point) = self.try_parse_measurement::<bool>(&msg) {
            Self::save_measurement(&db_client, point).await
          }        
        },
        None => (),
      }  
    }

    Ok(())
  
  }

  fn try_parse_measurement<'a, V: MeasurementValueTrait + Into<influxdb::Type>>(&'a self, msg: &'a StrMessage) -> Result<MeasurementPoint<'a, V>, serde_json::Error> {
    match serde_json::from_str::<Measurement<V>>(&msg.payload) {
      Ok(meas) => 
        Ok(MeasurementPoint {
          timestamp: meas.timestamp,
          measurement: &self.db_options.measurement,
          sensor: &msg.topic,
          value: meas.value,
        })
      ,
      Err(x) => Err(x)
    }
  }

  async fn save_measurement<'a, V: MeasurementValueTrait + Into<influxdb::Type>> (db_client: &influxdb::Client, point: MeasurementPoint<'a, V>) {
    let measurement =point.sensor;
    let timestamp = Timestamp::Milliseconds(point.timestamp.millisecond() as u128);

    let wq1 = influxdb::WriteQuery::new(
        timestamp, measurement,
      )
      .add_tag("sensor", point.sensor)
      .add_field("value", point.value.clone());
  
    let write_result = db_client
      .query(wq1)
      .await;

    match write_result {
      Ok(_) => {
        log::info!(
          "Writing of [{}] t = {}, v={:?} succeeded", 
          point.sensor, point.timestamp, point.value
        );
      },
      Err(e) => {
        log::error!(
          "Writing of [{}] t = {}, v={:?} failed: \n {}", 
          point.sensor, point.timestamp, point.value, e
        );
      },
    }
  }

}