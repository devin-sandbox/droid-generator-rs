use serde::{Serialize, Deserialize};

/// Gate/Trigger input parameter (0/1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateInput(pub String);

/// CV input parameter (voltage values)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CVInput(pub String);

/// Enumerated value parameter (mode selections)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumInput(pub String);

/// Boolean flag parameter (enable/disable features)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanInput(pub String);

/// Numeric range parameter (min/max bounds)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeInput(pub String);

/// Common input parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    Gate(GateInput),
    CV(CVInput),
    Enum(EnumInput),
    Boolean(BooleanInput),
    Range(RangeInput),
}

/// Common output parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputType {
    Gate,
    CV,
    Trigger,
}
