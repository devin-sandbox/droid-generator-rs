use serde::{Serialize, Deserialize};

mod input;
pub use input::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    P2B8,
    E4,
    M4,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::P2B8 => write!(f, "p2b8"),
            DeviceType::E4 => write!(f, "e4"),
            DeviceType::M4 => write!(f, "m4"),
        }
    }
}
