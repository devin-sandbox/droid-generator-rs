use crate::circuits::BaseCircuit;
use crate::error::{DroidError, Result};
use crate::types::DeviceType;
use crate::utils::ini_to_string;
use ini::Ini;

pub struct Patch {
    circuits: Vec<Box<dyn BaseCircuit>>,
    devices: Vec<DeviceType>,
}

impl Patch {
    /// Create a new patch with the specified devices
    pub fn new(devices: Vec<DeviceType>) -> Self {
        Self {
            circuits: Vec::new(),
            devices,
        }
    }
    
    /// Add a circuit to the patch
    pub fn add_circuit<T: BaseCircuit + 'static>(&mut self, circuit: T) {
        self.circuits.push(Box::new(circuit));
    }
    
    /// Convert the patch to an INI string
    pub fn to_string(&self) -> Result<String> {
        let mut output = String::from("# LABELS: master=18\n");
        let mut ini = Ini::new();
        
        // Add device sections
        for device in &self.devices {
            ini.with_section(Some(format!("{:?}", device))).set("", "");
        }
        
        // Add circuit sections
        for circuit in &self.circuits {
            let ini_str = circuit.to_ini()?;
            let section = circuit.section();
            
            // Parse the circuit's INI string and merge it
            let circuit_ini = Ini::load_from_str(&ini_str)
                .map_err(|e| DroidError::SerializationError(e.to_string()))?;
                
            if let Some(section_data) = circuit_ini.section(Some(section)) {
                for (key, value) in section_data.iter() {
                    ini.with_section(Some(section))
                       .set(key, value);
                }
            }
        }
        
        output.push_str(&ini_to_string(&ini)?);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_patch() {
        let devices = vec![DeviceType::P2B8];
        let patch = Patch::new(devices);
        assert_eq!(patch.circuits.len(), 0);
        assert_eq!(patch.devices.len(), 1);
    }
}
