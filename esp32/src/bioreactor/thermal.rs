use flowmbed_peripherals::channels::{MeasurementChannel, SetPointChannel, DeviceBusConnector, IOConnector};
use flowmbed_peripherals::util::QualifiedPath;
use flowmbed_peripherals::mqtt::MqttService;
use flowmbed_peripherals::set_value_handler;

pub struct ThermalController {
  pub temperature: f32,
  pub target_temperature: f32,
  pub temperature_tolerance: f32,
  pub heat_capacity: f32,
  pub heater_power: f32,
  pub heat_loss_conv: f32,
  pub ambient_temperature: f32,
  pub heater_on: bool,
}

impl ThermalController {
  pub fn new(init_temperature: f32) -> Self {
    ThermalController { 
      temperature: init_temperature, 
      target_temperature: 35.0,
      temperature_tolerance: 1.0, 
      heat_capacity: 3000.0, 
      heater_power: 100.0, 
      heat_loss_conv: 1.0, 
      ambient_temperature: 20.0, 
      heater_on: false 
    }
  }

  pub fn step(&mut self, _t_last: f32, dt: f32) {
    // Compute new state
    if (self.temperature < self.target_temperature - self.temperature_tolerance) && !self.heater_on {
      self.heater_on = true;
      println!("Turning heater on");
    } else if (self.temperature > self.target_temperature + self.temperature_tolerance) && self.heater_on {
      self.heater_on = false;
      println!("Turning heater off");
    }
    
    let q_dot = if self.heater_on {self.heater_power} else {0.0} -
      self.heat_loss_conv * (self.temperature - self.ambient_temperature);
    
    self.temperature += q_dot / self.heat_capacity * dt;
  }
}

pub struct ThermalBus {
  id: String,
  pub temperature: SetPointChannel,
  pub heater: MeasurementChannel<bool>,  
}

impl ThermalBus {
  pub fn new(id: &str) -> Self {
    ThermalBus {
      id: id.to_owned(),
      temperature: SetPointChannel::new("temperature"),
      heater: MeasurementChannel::new("heater")
    }
  }
}

impl DeviceBusConnector<ThermalController> for ThermalBus {
  fn sample(&self, device: &ThermalController) {
    self.temperature.current.sample(device.temperature);
    self.heater.sample(device.heater_on);
  }
  fn handle_actions(&self, device: &mut ThermalController) {
    set_value_handler!(self, temperature.target, device, target_temperature);
  }
}


impl IOConnector for ThermalBus {  
  fn connect_io(&mut self, comm: &mut dyn MqttService, qpath: &QualifiedPath) {
      self.temperature.connect_io(comm, &qpath.append(&self.id));
      self.heater.connect_io(comm, &qpath.append(&self.id));
  }
}