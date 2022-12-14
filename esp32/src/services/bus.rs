use esp_idf_svc::eventloop::EspBackgroundEventLoop;
use embedded_svc::mqtt;
use anyhow::Result;

fn start_eventloop() -> Result<EspBackgroundEventLoop> {
  let eventloop = EspBackgroundEventLoop::new(&Default::default())?;

  Ok(eventloop)
}