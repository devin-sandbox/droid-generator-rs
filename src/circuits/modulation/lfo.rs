use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::Result;
use crate::types::{GateInput, CVInput, BooleanInput};
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// LFO (Low Frequency Oscillator) circuit configuration
/// See DROID manual page 239
#[derive(Debug, Serialize, Deserialize)]
pub struct LFO {
    id: CircuitId,
    rate: Option<CVInput>,
    taptempo: Option<GateInput>,
    hz: Option<CVInput>,
    level: Option<CVInput>,
    randomize: Option<CVInput>,
    offset: Option<CVInput>,
    bipolar: Option<BooleanInput>,
    phase: Option<CVInput>,
    pulsewidth: Option<CVInput>,
    skew: Option<CVInput>,
    sync: Option<GateInput>,
    syncphase: Option<CVInput>,
    waveform: Option<CVInput>,
    output: Option<CVInput>,
    square: Option<CVInput>,
    sawtooth: Option<CVInput>,
    triangle: Option<CVInput>,
    ramp: Option<CVInput>,
    paraboloid: Option<CVInput>,
    sine: Option<CVInput>,
    cosine: Option<CVInput>,
}

impl LFO {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            rate: None,
            taptempo: None,
            hz: None,
            level: None,
            randomize: None,
            offset: None,
            bipolar: None,
            phase: None,
            pulsewidth: None,
            skew: None,
            sync: None,
            syncphase: None,
            waveform: None,
            output: None,
            square: None,
            sawtooth: None,
            triangle: None,
            ramp: None,
            paraboloid: None,
            sine: None,
            cosine: None,
        }
    }

    pub fn with_rate(mut self, rate: impl Into<String>) -> Self {
        self.rate = Some(CVInput(rate.into()));
        self
    }

    pub fn with_hz(mut self, hz: impl Into<String>) -> Self {
        self.hz = Some(CVInput(hz.into()));
        self
    }

    pub fn with_output(mut self, output: impl Into<String>) -> Self {
        self.output = Some(CVInput(output.into()));
        self
    }

    pub fn with_sawtooth(mut self, sawtooth: impl Into<String>) -> Self {
        self.sawtooth = Some(CVInput(sawtooth.into()));
        self
    }

    pub fn with_level(mut self, level: impl Into<String>) -> Self {
        self.level = Some(CVInput(level.into()));
        self
    }
}

impl BaseCircuit for LFO {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "lfo"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        if let Some(rate) = &self.rate {
            ini.with_section(section).set("rate", &rate.0);
        }
        if let Some(taptempo) = &self.taptempo {
            ini.with_section(section).set("taptempo", &taptempo.0);
        }
        if let Some(hz) = &self.hz {
            ini.with_section(section).set("hz", &hz.0);
        }
        if let Some(level) = &self.level {
            ini.with_section(section).set("level", &level.0);
        }
        if let Some(randomize) = &self.randomize {
            ini.with_section(section).set("randomize", &randomize.0);
        }
        if let Some(offset) = &self.offset {
            ini.with_section(section).set("offset", &offset.0);
        }
        if let Some(bipolar) = &self.bipolar {
            ini.with_section(section).set("bipolar", &bipolar.0);
        }
        if let Some(phase) = &self.phase {
            ini.with_section(section).set("phase", &phase.0);
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
        if let Some(ramp) = &self.ramp {
            ini.with_section(section).set("ramp", &ramp.0);
        }
        if let Some(paraboloid) = &self.paraboloid {
            ini.with_section(section).set("paraboloid", &paraboloid.0);
        }
        if let Some(sine) = &self.sine {
            ini.with_section(section).set("sine", &sine.0);
        }
        if let Some(cosine) = &self.cosine {
            ini.with_section(section).set("cosine", &cosine.0);
        }

        ini_to_string(&ini)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfo_creation() {
        let lfo = LFO::new("lfo1")
            .with_rate("1V")
            .with_hz("440");
        
        assert_eq!(lfo.id().to_string(), "lfo1");
        assert_eq!(lfo.section(), "lfo");
    }
}
