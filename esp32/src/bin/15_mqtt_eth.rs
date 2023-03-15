use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio;
use esp_idf_hal::spi;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use flowmbed_esp32::services;
use log::*;
use serde::Serialize;
use esp_idf_svc::eth;
use esp_idf_hal::units;
use std::time::Duration;
use embedded_svc::ipv4;
use esp_idf_svc::netif::{EspNetif, EspNetifWait};
use esp_idf_svc::ping;

#[derive(Serialize)]
struct MyPayload {
  device: String,
  i: u32,
  x: f32,
  y: f32
}


fn ping(ip: ipv4::Ipv4Addr) -> anyhow::Result<()> {
  info!("About to do some pings for {:?}", ip);

  let ping_summary = ping::EspPing::default().ping(ip, &Default::default())?;
  if ping_summary.transmitted != ping_summary.received {
      anyhow::bail!("Pinging IP {} resulted in timeouts", ip);
  }

  info!("Pinging done");

  Ok(())
}

fn eth_configure(
  sysloop: &EspSystemEventLoop,
  mut eth: Box<esp_idf_svc::eth::EspEth<'static>>,
) -> anyhow::Result<Box<esp_idf_svc::eth::EspEth<'static>>> {
  use std::net::Ipv4Addr;

  info!("Eth created");

  eth.start()?;

  info!("Starting eth...");

  if !esp_idf_svc::eth::EthWait::new(eth.driver(), sysloop)?
      .wait_with_timeout(Duration::from_secs(20), || eth.is_started().unwrap())
  {
      anyhow::bail!("Eth did not start");
  }

  if !EspNetifWait::new::<EspNetif>(eth.netif(), &sysloop)?
      .wait_with_timeout(Duration::from_secs(20), || {
          eth.netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
      })
  {
    anyhow::bail!("Eth did not receive a DHCP lease");
  }

  let ip_info = eth.netif().get_ip_info()?;

  info!("Eth DHCP info: {:?}", ip_info);

  //ping(ip_info.subnet.gateway)?;

  Ok(eth)
}

fn test_ethernet() -> anyhow::Result<Box<esp_idf_svc::eth::EspEth<'static>>> {
  let internal_peripherals = Peripherals::take()
    .ok_or(anyhow::anyhow!("Peripherals already taken"))?;

  let sysloop = EspSystemEventLoop::take()?;
  // let cs: Option<gpio::Gpio14> = None; //Some(internal_peripherals.pins.gpio14);
  // let rst: Option<gpio::Gpio25> = None; //Some(internal_peripherals.pins.gpio25);

  info!("Initializing driver");

  // let driver = eth::EthDriver::new_spi(
  //   internal_peripherals.spi2, // VSPI
  //   internal_peripherals.pins.gpio16, // int (rx2)
  //   internal_peripherals.pins.gpio14, // sclk
  //   internal_peripherals.pins.gpio13, // sdo
  //   internal_peripherals.pins.gpio12, // sdi
  //   esp_idf_hal::spi::Dma::Auto(4096),
  //   Some(internal_peripherals.pins.gpio15), // cs
  //   Some(internal_peripherals.pins.gpio17), // rst (tx2)
  //   eth::SpiEthChipset::W5500,
  //   units::MegaHertz(10).into(),
  //   Some(&[0x02, 0x00, 0x00, 0x12, 0x34, 0x56]),
  //   None,
  //   sysloop.clone()
  // )?;

  let driver = eth::EthDriver::new_spi(
    internal_peripherals.spi2, // VSPI
    internal_peripherals.pins.gpio16, // int (rx2)
    internal_peripherals.pins.gpio14, // sclk
    internal_peripherals.pins.gpio13, // sdo
    internal_peripherals.pins.gpio12, // sdi
    esp_idf_hal::spi::Dma::Auto(4096),
    Some(internal_peripherals.pins.gpio15), // cs
    Some(internal_peripherals.pins.gpio17), // rst (tx2)
    eth::SpiEthChipset::W5500,
    units::MegaHertz(10).into(),
    Some(&[0x02, 0x00, 0x00, 0x12, 0x34, 0x56]),
    None,
    sysloop.clone()
  )?;

  info!("Initialized driver");

  let eth = eth_configure(
    &sysloop,
    Box::new(eth::EspEth::wrap(driver)?)
  )?;

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
    info!("{}", payload_str);
    mqtt.publish_obj(&payload);
    i += 1;

  }

  Ok(eth)
}

pub fn main() {
  esp_idf_svc::log::EspLogger::initialize_default();
  test_ethernet().unwrap();
}