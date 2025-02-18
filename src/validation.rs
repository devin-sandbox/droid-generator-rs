use crate::error::{DroidError, Result};
// No imports needed for validation functions

pub trait CircuitValidator {
    fn validate(&self) -> Result<()>;
}

pub fn validate_cv_range(value: &str, min: f64, max: f64, param: &str) -> Result<()> {
    if let Ok(v) = value.parse::<f64>() {
        if v < min || v > max {
            return Err(DroidError::ValidationError(
                format!("Parameter '{}' value {} outside valid range [{}, {}]", param, v, min, max)
            ));
        }
    }
    Ok(())
}

pub fn validate_gate(value: &str, param: &str) -> Result<()> {
    match value {
        "0" | "1" => Ok(()),
        _ => Err(DroidError::ValidationError(
            format!("Invalid gate value '{}' for parameter '{}', must be 0 or 1", value, param)
        ))
    }
}

pub fn validate_enum(value: &str, valid_values: &[&str], param: &str) -> Result<()> {
    if !valid_values.contains(&value) {
        return Err(DroidError::ValidationError(
            format!("Invalid value '{}' for parameter '{}', must be one of: {:?}", 
                   value, param, valid_values)
        ));
    }
    Ok(())
}

pub fn validate_boolean(value: &str, param: &str) -> Result<()> {
    match value {
        "0" | "1" | "true" | "false" => Ok(()),
        _ => Err(DroidError::ValidationError(
            format!("Invalid boolean value '{}' for parameter '{}', must be 0/1 or true/false", 
                   value, param)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cv_range_validation() {
        assert!(validate_cv_range("0.5", 0.0, 1.0, "test").is_ok());
        assert!(validate_cv_range("1.5", 0.0, 1.0, "test").is_err());
    }

    #[test]
    fn test_gate_validation() {
        assert!(validate_gate("0", "test").is_ok());
        assert!(validate_gate("1", "test").is_ok());
        assert!(validate_gate("2", "test").is_err());
    }

    #[test]
    fn test_enum_validation() {
        let valid = &["a", "b", "c"];
        assert!(validate_enum("a", valid, "test").is_ok());
        assert!(validate_enum("d", valid, "test").is_err());
    }

    #[test]
    fn test_boolean_validation() {
        assert!(validate_boolean("0", "test").is_ok());
        assert!(validate_boolean("1", "test").is_ok());
        assert!(validate_boolean("true", "test").is_ok());
        assert!(validate_boolean("false", "test").is_ok());
        assert!(validate_boolean("2", "test").is_err());
    }
}
