use serde::{Serialize, Deserialize};
use super::peripheral::PeripheralConfig;
use super::system::SystemConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardConfig {
    peripherals: Vec<PeripheralConfig>,
    systems: Vec<SystemConfig>,
}