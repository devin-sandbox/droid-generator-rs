use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::{DroidError, Result};
use crate::types::{GateInput, CVInput, BooleanInput};
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// Sequencer circuit configuration
/// See DROID manual page 320
#[derive(Debug, Serialize, Deserialize)]
pub struct Sequencer {
    id: CircuitId,
    // Clock input
    clock: Option<GateInput>,
    // Reset trigger
    reset: Option<GateInput>,
    // Direction control
    direction: Option<CVInput>,
    // Length control
    length: Option<CVInput>,
    // Step values (1-16)
    value1: Option<CVInput>,
    value2: Option<CVInput>,
    value3: Option<CVInput>,
    value4: Option<CVInput>,
    value5: Option<CVInput>,
    value6: Option<CVInput>,
    value7: Option<CVInput>,
    value8: Option<CVInput>,
    value9: Option<CVInput>,
    value10: Option<CVInput>,
    value11: Option<CVInput>,
    value12: Option<CVInput>,
    value13: Option<CVInput>,
    value14: Option<CVInput>,
    value15: Option<CVInput>,
    value16: Option<CVInput>,
    // Gate values (1-16)
    gate1: Option<BooleanInput>,
    gate2: Option<BooleanInput>,
    gate3: Option<BooleanInput>,
    gate4: Option<BooleanInput>,
    gate5: Option<BooleanInput>,
    gate6: Option<BooleanInput>,
    gate7: Option<BooleanInput>,
    gate8: Option<BooleanInput>,
    gate9: Option<BooleanInput>,
    gate10: Option<BooleanInput>,
    gate11: Option<BooleanInput>,
    gate12: Option<BooleanInput>,
    gate13: Option<BooleanInput>,
    gate14: Option<BooleanInput>,
    gate15: Option<BooleanInput>,
    gate16: Option<BooleanInput>,
    // Outputs
    output: Option<CVInput>,
    gate: Option<GateInput>,
    step: Option<CVInput>,
}

impl Sequencer {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            clock: None,
            reset: None,
            direction: None,
            length: None,
            value1: None,
            value2: None,
            value3: None,
            value4: None,
            value5: None,
            value6: None,
            value7: None,
            value8: None,
            value9: None,
            value10: None,
            value11: None,
            value12: None,
            value13: None,
            value14: None,
            value15: None,
            value16: None,
            gate1: None,
            gate2: None,
            gate3: None,
            gate4: None,
            gate5: None,
            gate6: None,
            gate7: None,
            gate8: None,
            gate9: None,
            gate10: None,
            gate11: None,
            gate12: None,
            gate13: None,
            gate14: None,
            gate15: None,
            gate16: None,
            output: None,
            gate: None,
            step: None,
        }
    }

    pub fn with_value(&mut self, index: u8, value: impl Into<String>) -> &mut Self {
        match index {
            1 => self.value1 = Some(CVInput(value.into())),
            2 => self.value2 = Some(CVInput(value.into())),
            // ... implement for all values
            _ => {}
        }
        self
    }

    pub fn with_gate(&mut self, index: u8, value: impl Into<String>) -> &mut Self {
        match index {
            1 => self.gate1 = Some(BooleanInput(value.into())),
            2 => self.gate2 = Some(BooleanInput(value.into())),
            // ... implement for all gates
            _ => {}
        }
        self
    }
}

impl BaseCircuit for Sequencer {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "sequencer"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        // Serialize all fields following DROID manual specifications
        if let Some(clock) = &self.clock {
            ini.with_section(section).set("clock", &clock.0);
        }
        if let Some(reset) = &self.reset {
            ini.with_section(section).set("reset", &reset.0);
        }
        if let Some(direction) = &self.direction {
            ini.with_section(section).set("direction", &direction.0);
        }
        if let Some(length) = &self.length {
            ini.with_section(section).set("length", &length.0);
        }

        // Serialize step values
        for i in 1..=16 {
            if let Some(value) = self.get_value(i) {
                ini.with_section(section).set(&format!("value{}", i), &value.0);
            }
        }

        // Serialize gate values
        for i in 1..=16 {
            if let Some(gate) = self.get_gate(i) {
                ini.with_section(section).set(&format!("gate{}", i), &gate.0);
            }
        }

        // Serialize outputs
        if let Some(output) = &self.output {
            ini.with_section(section).set("output", &output.0);
        }
        if let Some(gate) = &self.gate {
            ini.with_section(section).set("gate", &gate.0);
        }
        if let Some(step) = &self.step {
            ini.with_section(section).set("step", &step.0);
        }

        ini_to_string(&ini)
    }
}

impl Sequencer {
    fn get_value(&self, index: u8) -> Option<&CVInput> {
        match index {
            1 => self.value1.as_ref(),
            2 => self.value2.as_ref(),
            // ... implement for all values
            _ => None
        }
    }

    fn get_gate(&self, index: u8) -> Option<&BooleanInput> {
        match index {
            1 => self.gate1.as_ref(),
            2 => self.gate2.as_ref(),
            // ... implement for all gates
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequencer_creation() {
        let mut seq = Sequencer::new("seq1");
        seq.with_value(1, "1V")
           .with_gate(1, "1");
        
        assert_eq!(seq.id().to_string(), "seq1");
        assert_eq!(seq.section(), "sequencer");
    }
}
