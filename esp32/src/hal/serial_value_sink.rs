use flowmbed_peripherals::sinks::traits::ValueSink;
use flowmbed_dynsys::data::Value;

pub struct SerialValueSink {

}

impl SerialValueSink {

}

impl ValueSink for SerialValueSink {
  fn send(&mut self, v: Value) -> anyhow::Result<()> {
    println!("{:?}", v);
    Ok(())
  }
}