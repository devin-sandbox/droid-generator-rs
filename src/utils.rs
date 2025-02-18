use ini::Ini;
use crate::error::{DroidError, Result};

pub fn ini_to_string(ini: &Ini) -> Result<String> {
    let mut buffer = Vec::new();
    ini.write_to(&mut buffer)
        .map_err(|e| DroidError::SerializationError(e.to_string()))?;
    
    String::from_utf8(buffer)
        .map_err(|e| DroidError::SerializationError(e.to_string()))
}
