use flowmbed_peripherals::set_value_handler;

fn handle_actions(&self, device: &mut StirrerModel) {
  set_value_handler!(self, on_time, device, on_time);
  set_value_handler!(self, off_time, device, off_time);
  // set_value_handler!(self.speed, target, device, omega_target);
}

fn main() {
  // set_value_handler!(self, speed.target, device, on_time);
}