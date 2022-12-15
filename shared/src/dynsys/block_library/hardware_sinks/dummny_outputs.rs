use embedded_hal::digital;

pub struct DummyDigitalOutput {

}
    
impl digital::ErrorType for DummyDigitalOutput {
  type Error = anyhow::Error;
}

impl digital::OutputPin for DummyDigitalOutput {

  fn set_high(&mut self) -> Result<(), Self::Error> {
    println!("LED is on");
    Ok(())
  }
  fn set_low(&mut self) -> Result<(), Self::Error> {
    println!("LED is off");
    Ok(())
  }
}