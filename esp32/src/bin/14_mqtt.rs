use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use flowmbed_esp32::services;
use log::*;
use serde::Serialize;

#[derive(Serialize)]
struct MyPayload {
  device: String,
  i: u32,
  x: f32,
  y: f32
}

fn test_wifi() -> anyhow::Result<()> {
  //esp_get_free_heap_size
  let internal_peripherals = Peripherals::take().unwrap();
  let sysloop = EspSystemEventLoop::take()?;
  
  let wifi = services::wifi::start(internal_peripherals.modem, sysloop.clone())?;
  info!("Initialized wifi");

  // let mut indicator_led = gpio::PinDriver::output(internal_peripherals.pins.gpio2)?;
  // indicator_led.set_high()?;
  
  // let sysloop = EspSystemEventLoop::take()?;
  // info!("Initialized sysloop");
  
  let mut mqtt = services::MqttService::new()?;
  info!("Initialized MQTT");

  let mut i = 0;
  loop {
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let device = "node1".to_owned();
    let x = 0.123 + (i as f32) * 0.01;
    let y = 0.543 + (i as f32) * 0.1; 
    let payload = MyPayload {
      device, i, x, y
    };

    //myMeasurement,tag1=value1,tag2=value2 fieldKey="fieldValue"
    let payload_str=format!("measured_data,device=\"node1\" i={i},x={x},y={y}");
    mqtt.publish_obj(&payload);
    i += 1;
  }

  Ok(())
}

pub fn main() {
  esp_idf_svc::log::EspLogger::initialize_default();
  test_wifi().unwrap()
}