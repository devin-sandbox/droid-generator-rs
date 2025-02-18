use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::{DroidError, Result};
use crate::types::{CVInput, GateInput, BooleanInput};
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// VCO (Voltage Controlled Oscillator) circuit configuration
/// See DROID manual page 245
#[derive(Debug, Serialize, Deserialize)]
pub struct VCO {
    id: CircuitId,
    // Pitch control
    pitch: Option<CVInput>,
    // Frequency in Hz
    hz: Option<CVInput>,
    // Output level
    level: Option<CVInput>,
    // Pulse width
    pulsewidth: Option<CVInput>,
    // Waveform skew
    skew: Option<CVInput>,
    // Sync input
    sync: Option<GateInput>,
    // Sync phase
    syncphase: Option<CVInput>,
    // Waveform selection
    waveform: Option<CVInput>,
    // Outputs
    output: Option<CVInput>,
    square: Option<CVInput>,
    sawtooth: Option<CVInput>,
    triangle: Option<CVInput>,
    sine: Option<CVInput>,
    // Tuning
    tuningmode: Option<BooleanInput>,
    tuningpitch: Option<CVInput>,
    transpose: Option<CVInput>,
}

impl VCO {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            pitch: None,
            hz: None,
            level: None,
            pulsewidth: None,
            skew: None,
            sync: None,
            syncphase: None,
            waveform: None,
            output: None,
            square: None,
            sawtooth: None,
            triangle: None,
            sine: None,
            tuningmode: None,
            tuningpitch: None,
            transpose: None,
        }
    }

    pub fn with_pitch(mut self, pitch: impl Into<String>) -> Self {
        self.pitch = Some(CVInput(pitch.into()));
        self
    }

    pub fn with_hz(mut self, hz: impl Into<String>) -> Self {
        self.hz = Some(CVInput(hz.into()));
        self
    }

    // Add other builder methods as needed
}

impl BaseCircuit for VCO {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "vco"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        if let Some(pitch) = &self.pitch {
            ini.with_section(section).set("pitch", &pitch.0);
        }
        if let Some(hz) = &self.hz {
            ini.with_section(section).set("hz", &hz.0);
        }
        if let Some(level) = &self.level {
            ini.with_section(section).set("level", &level.0);
        }
        if let Some(pulsewidth) = &self.pulsewidth {
            ini.with_section(section).set("pulsewidth", &pulsewidth.0);
        }
        if let Some(skew) = &self.skew {
            ini.with_section(section).set("skew", &skew.0);
        }
        if let Some(sync) = &self.sync {
            ini.with_section(section).set("sync", &sync.0);
        }
        if let Some(syncphase) = &self.syncphase {
            ini.with_section(section).set("syncphase", &syncphase.0);
        }
        if let Some(waveform) = &self.waveform {
            ini.with_section(section).set("waveform", &waveform.0);
        }
        if let Some(output) = &self.output {
            ini.with_section(section).set("output", &output.0);
        }
        if let Some(square) = &self.square {
            ini.with_section(section).set("square", &square.0);
        }
        if let Some(sawtooth) = &self.sawtooth {
            ini.with_section(section).set("sawtooth", &sawtooth.0);
        }
        if let Some(triangle) = &self.triangle {
            ini.with_section(section).set("triangle", &triangle.0);
        }
        if let Some(sine) = &self.sine {
            ini.with_section(section).set("sine", &sine.0);
        }
        if let Some(tuningmode) = &self.tuningmode {
            ini.with_section(section).set("tuningmode", &tuningmode.0);
        }
        if let Some(tuningpitch) = &self.tuningpitch {
            ini.with_section(section).set("tuningpitch", &tuningpitch.0);
        }
        if let Some(transpose) = &self.transpose {
            ini.with_section(section).set("transpose", &transpose.0);
        }

        ini_to_string(&ini)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vco_creation() {
        let vco = VCO::new("vco1")
            .with_pitch("1V")
            .with_hz("440");
        
        assert_eq!(vco.id().to_string(), "vco1");
        assert_eq!(vco.section(), "vco");
    }
}
