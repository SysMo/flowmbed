use log::*;
use anyhow::bail;

use embedded_svc::wifi::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::netif::{EspNetif, EspNetifWait};
use esp_idf_hal::peripheral;
use std::time::Duration;
use anyhow::Result;

struct Config {
  ssid: &'static str,
  password: &'static str,
  channel: Option<u8>
}

static config: Config = Config {
  ssid: env!("ESP32_WIFI_SSID"),
  password: env!("ESP32_WIFI_PASS"),
  channel: Some(1)
};

pub fn start(
  modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
  sysloop: EspSystemEventLoop,
) -> Result<Box<EspWifi<'static>>> {
  use std::net::Ipv4Addr;

  use esp_idf_svc::handle::RawHandle;

  let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), None)?);
  info!("Wifi created, about to scan");
  info!("MAC: {:?}", wifi.sta_netif().get_mac());

  let ap_infos = wifi.scan()?;

  let ours = ap_infos.into_iter().find(|a| a.ssid == config.ssid);

  let net = match ours {
      Some(net) => net,
      None => {
        
          error!(
            "Configured access point {} not found during scanning", config.ssid
          );
          bail!(
            "Configured access point {} not found during scanning", config.ssid
          )
      }
  };

  let channel = Some(config.channel.unwrap_or(net.channel));
//   let auth_method = net.auth_method;
  info!(
    "Found configured access point {} on channel {}",
    config.ssid, net.channel);

  wifi.set_configuration(&Configuration::Client(
    // ClientConfiguration {
    //   ssid: config.ssid.into(),
    //   password: config.password.into(),
    //   auth_method: AuthMethod::None,
    //   ..Default::default()
    // },

      ClientConfiguration {
          ssid: config.ssid.into(),
          password: config.password.into(),
          ..Default::default()
      }

    //   AccessPointConfiguration {
    //       ssid: "aptest".into(),
    //       channel: channel.unwrap_or(1),
    //       ..Default::default()
    //   },
  ))?;

  wifi.start()?;

  info!("Starting wifi...");

  if !WifiWait::new(&sysloop)?
      .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
  {
      bail!("Wifi did not start");
  }

  info!("Connecting wifi...");

  wifi.connect()?;

  if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
      Duration::from_secs(20),
      || {
          wifi.is_connected().unwrap()
              && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
      },
  ) {
      bail!("Wifi did not connect or did not receive a DHCP lease");
  }

  let ip_info = wifi.sta_netif().get_ip_info()?;

  info!("Wifi DHCP info: {:?}", ip_info);

  Ok(wifi)
}