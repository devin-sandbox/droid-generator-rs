use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DroidError {
    InvalidCircuitConfig(String),
    SerializationError(String),
    ValidationError(String),
    InvalidParameterValue { param: String, value: String, reason: String },
    UnsupportedCircuitType(String),
    CircuitNotFound(String),
    InvalidConfiguration(String),
}

impl fmt::Display for DroidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DroidError::InvalidCircuitConfig(msg) => write!(f, "Invalid circuit configuration: {}", msg),
            DroidError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            DroidError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DroidError::InvalidParameterValue { param, value, reason } => 
                write!(f, "Invalid value '{}' for parameter '{}': {}", value, param, reason),
            DroidError::UnsupportedCircuitType(msg) => write!(f, "Unsupported circuit type: {}", msg),
            DroidError::CircuitNotFound(msg) => write!(f, "Circuit not found: {}", msg),
            DroidError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl Error for DroidError {}

pub type Result<T> = std::result::Result<T, DroidError>;
