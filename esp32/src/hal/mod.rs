mod serial_value_sink;

pub use serial_value_sink::SerialValueSink;

mod adc;
mod digital;

pub use adc::ADCReader;
pub use adc::ESP32_AnalogReaderMultiChannel as AnalogReaderMultiChannel;
pub use adc::ESP32_AnalogChannel as AnalogChannel;
pub use adc::ESP32_AnalogChannelReader as AnalogChannelReader;
pub use digital::DigitalInputPin;
pub use digital::DigitalOutputPin;

