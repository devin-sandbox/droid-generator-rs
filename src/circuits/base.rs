use serde::{Serialize, Deserialize};
use crate::error::Result;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitId(String);

impl CircuitId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for CircuitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Base trait for all DROID circuits
/// All circuit implementations must follow DROID manual specifications
pub trait BaseCircuit {
    /// Get the unique identifier for this circuit instance
    fn id(&self) -> &CircuitId;
    
    /// Get the section name for this circuit type in the INI file
    fn section(&self) -> &str;
    
    /// Convert the circuit to its INI representation
    fn to_ini(&self) -> Result<String>;
}

/// Trait for circuits that support tuning capabilities
pub trait TuningCircuit: BaseCircuit {
    fn tuning_mode(&self) -> Option<&str>;
    fn tuning_pitch(&self) -> Option<&str>;
    fn transpose(&self) -> Option<&str>;
}

/// Trait for circuits that support MIDI channel selection
pub trait MidiChannelCircuit: BaseCircuit {
    fn channel(&self) -> Option<&str>;
}
