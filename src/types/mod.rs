use serde::{Serialize, Deserialize};

mod input;
pub use input::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    P2B8,
    E4,
    M4,
}
