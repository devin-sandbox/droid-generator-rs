use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::{DroidError, Result};
use crate::types::{GateInput, CVInput, BooleanInput};
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// Clock tool circuit configuration
/// See DROID manual page 330
#[derive(Debug, Serialize, Deserialize)]
pub struct ClockTool {
    id: CircuitId,
    // Clock input
    clock: Option<GateInput>,
    // Reset trigger
    reset: Option<GateInput>,
    // Run gate
    run: Option<GateInput>,
    // Tempo in BPM
    bpm: Option<CVInput>,
    // Multiplier
    multiply: Option<CVInput>,
    // Divider
    divide: Option<CVInput>,
    // Swing amount
    swing: Option<CVInput>,
    // Shuffle amount
    shuffle: Option<CVInput>,
    // Outputs
    output: Option<GateInput>,
    running: Option<GateInput>,
    start: Option<GateInput>,
    stop: Option<GateInput>,
}

impl ClockTool {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            clock: None,
            reset: None,
            run: None,
            bpm: None,
            multiply: None,
            divide: None,
            swing: None,
            shuffle: None,
            output: None,
            running: None,
            start: None,
            stop: None,
        }
    }

    pub fn with_bpm(mut self, bpm: impl Into<String>) -> Self {
        self.bpm = Some(CVInput(bpm.into()));
        self
    }

    pub fn with_multiply(mut self, multiply: impl Into<String>) -> Self {
        self.multiply = Some(CVInput(multiply.into()));
        self
    }
}

impl BaseCircuit for ClockTool {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "clocktool"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        if let Some(clock) = &self.clock {
            ini.with_section(section).set("clock", &clock.0);
        }
        if let Some(reset) = &self.reset {
            ini.with_section(section).set("reset", &reset.0);
        }
        if let Some(run) = &self.run {
            ini.with_section(section).set("run", &run.0);
        }
        if let Some(bpm) = &self.bpm {
            ini.with_section(section).set("bpm", &bpm.0);
        }
        if let Some(multiply) = &self.multiply {
            ini.with_section(section).set("multiply", &multiply.0);
        }
        if let Some(divide) = &self.divide {
            ini.with_section(section).set("divide", &divide.0);
        }
        if let Some(swing) = &self.swing {
            ini.with_section(section).set("swing", &swing.0);
        }
        if let Some(shuffle) = &self.shuffle {
            ini.with_section(section).set("shuffle", &shuffle.0);
        }
        if let Some(output) = &self.output {
            ini.with_section(section).set("output", &output.0);
        }
        if let Some(running) = &self.running {
            ini.with_section(section).set("running", &running.0);
        }
        if let Some(start) = &self.start {
            ini.with_section(section).set("start", &start.0);
        }
        if let Some(stop) = &self.stop {
            ini.with_section(section).set("stop", &stop.0);
        }

        ini_to_string(&ini)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clocktool_creation() {
        let clock = ClockTool::new("clock1")
            .with_bpm("120")
            .with_multiply("2");
        
        assert_eq!(clock.id().to_string(), "clock1");
        assert_eq!(clock.section(), "clocktool");
    }
}
