use super::BioreactorPeripherals;
use super::thermal::{ThermalController, ThermalBus};
use super::stirrer::{StirrerController, StirrerBus};
use flowmbed_peripherals::channels::{DeviceBusConnector, IOConnector};
use flowmbed_peripherals::util::QualifiedPath;
use flowmbed_peripherals::mqtt::MqttService;

pub struct BioreactorController<'a> {
  thermal: ThermalController,
  stirrer: StirrerController<'a>,
}

impl<'a> BioreactorController<'a> {
  pub fn new(peripherals: BioreactorPeripherals<'a>) -> Self {
    BioreactorController { 
      thermal: ThermalController::new(30.0),
      stirrer: StirrerController::new(peripherals.stirrer) 
    }
  }

  pub fn step(&mut self, t_last: f32, dt: f32) {
    self.thermal.step(t_last, dt);
    self.stirrer.step(t_last, dt);
  }
}

pub struct BioreactorBus {
  id: String,
  thermal: ThermalBus,
  stirrer: StirrerBus,
}

impl BioreactorBus {
  pub fn new(id: &str) -> Self {
    BioreactorBus { 
      id: id.to_owned(),
      thermal: ThermalBus::new("thermal"),
      stirrer: StirrerBus::new("stirrer"),
    }
  }
}

impl<'a> DeviceBusConnector<BioreactorController<'a>> for BioreactorBus {
  fn sample(&self, device: &BioreactorController) {
    self.thermal.sample(&device.thermal);
    self.stirrer.sample(&device.stirrer);
  }
  fn handle_actions(&self, device: &mut BioreactorController) {
    self.thermal.handle_actions(&mut device.thermal);
    self.stirrer.handle_actions(&mut device.stirrer);
    
  }
}

impl IOConnector for BioreactorBus {  
  fn connect_io(&mut self, comm: &mut dyn MqttService, qpath: &QualifiedPath) {
      self.thermal.connect_io(comm, &qpath.append(&self.id));
      self.stirrer.connect_io(comm, &qpath.append(&self.id));
  }
}

