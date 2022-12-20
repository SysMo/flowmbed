use serde::{Serialize, Deserialize};
use super::peripheral::PeripheralConfig;
use super::system::SystemConfig;
use super::circuit::Circuit;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Device {
    esp32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardConfig {
    device: Device, 
    peripherals: Vec<PeripheralConfig>,
    circuits: Vec<Circuit>,
    // systems: Vec<SystemConfig>,
}