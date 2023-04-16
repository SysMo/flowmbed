use flowmbed_esp32::bioreactor::{BioreactorController, BioreactorBus, BioreactorPeripherals, ThermalPeripherals, StirrerPeripherals};
use flowmbed_esp32::services::{MqttServiceImpl, wifi};
use flowmbed_peripherals::channels::{DeviceBusConnector, IOConnector};
use flowmbed_peripherals::mqtt::{MqttServiceOptions};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::prelude::*;
use esp_idf_hal::i2c::{I2cConfig};
use esp_idf_svc::eventloop::EspSystemEventLoop;

// use ads1x1x::interface::I2cInterface;
// use ads1x1x::{channel, Ads1x1x, SlaveAddr, FullScaleRange};


fn main() {
  esp_idf_svc::log::EspLogger::initialize_default();

  let internal_peripherals = Peripherals::take().unwrap();
  let sysloop = EspSystemEventLoop::take().unwrap();
  // Start wifi
  let wifi = wifi::start(internal_peripherals.modem, sysloop.clone()).unwrap();
  log::info!("Initialized wifi");

  use flowmbed_peripherals::sensors::ads1x1x;
  // Create the peripherals
  let mut ads1115 = {
    let i2c = internal_peripherals.i2c0;
    let sda = internal_peripherals.pins.gpio21;
    let scl = internal_peripherals.pins.gpio22;
  
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();
  
    ads1x1x::Ads1x1xDeviceConfigurator::new(
      i2c, ads1x1x::FullScaleRange::Within4_096V
    ).add_channel(ads1x1x::ChannelSelection::DifferentialA0A1)
    .build().split()
    
  };

  use esp_idf_hal::ledc;
  use flowmbed_peripherals::actuators::traits::PwmMultiChannel;
  // 1000.Hz(), internal_peripherals.ledc.timer0
  let mut pwm_out = flowmbed_esp32::hal::PwmMultiChannel::<1>::builder(
    1000.Hz(), internal_peripherals.ledc.timer0
  ).unwrap().add_channel(internal_peripherals.ledc.channel0, internal_peripherals.pins.gpio16
  ).unwrap().build().unwrap();

  let bioreactor_peripherals = BioreactorPeripherals {
    thermal: ThermalPeripherals {},
    stirrer: StirrerPeripherals {
        speed_reader: &mut ads1115[0],
        speed_pwm: pwm_out.channel(0).unwrap(),
    },
  };

  let mqtt_options = MqttServiceOptions {
    host: "mqtts://c0a7164f.ala.us-east-1.emqxsl.com:8883".to_owned(),
    client_id: "rust_paho_1".to_owned(),
    user: "sysmo".to_owned(), 
    password: "sysmopass".to_owned(),
  };
  MqttServiceImpl::with(mqtt_options, |mqtt| {
    let mut bioreactor = BioreactorController::new(bioreactor_peripherals);
    let mut device_bus = BioreactorBus::new("bioreactor");
    device_bus.connect_io_root(mqtt);    
  
    let mut t: f32 = 0.0; 
    let dt: f32 = 1.0;
    loop {
      device_bus.handle_actions(&mut bioreactor);
      bioreactor.step(t, dt);
      t += dt;
      device_bus.sample(&bioreactor);
      // println!("t = {t}s");
      std::thread::sleep(std::time::Duration::from_millis((dt*1000.0) as u64));
    }
  });

    

}