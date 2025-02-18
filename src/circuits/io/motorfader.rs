use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::{DroidError, Result};
use crate::types::{GateInput, CVInput, BooleanInput};
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// Motor fader circuit configuration
/// See DROID manual page 311
#[derive(Debug, Serialize, Deserialize)]
pub struct MotorFader {
    id: CircuitId,
    fader: Option<CVInput>,
    sharewithnext: Option<BooleanInput>,
    ledvalue: Option<CVInput>,
    ledcolor: Option<CVInput>,
    startvalue: Option<CVInput>,
    notches: Option<CVInput>,
    outputscale: Option<CVInput>,
    outputoffset: Option<CVInput>,
    smooth: Option<CVInput>,
    snapto: Option<CVInput>,
    snapforce: Option<CVInput>,
    color: Option<CVInput>,
    negativecolor: Option<CVInput>,
    ledfill: Option<BooleanInput>,
    output: Option<CVInput>,
    button: Option<GateInput>,
    selectat: Option<CVInput>,
    preset: Option<CVInput>,
    loadpreset: Option<GateInput>,
    savepreset: Option<GateInput>,
    clear: Option<GateInput>,
    clearall: Option<GateInput>,
    dontsave: Option<BooleanInput>,
}

impl MotorFader {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            fader: None,
            sharewithnext: None,
            ledvalue: None,
            ledcolor: None,
            startvalue: None,
            notches: None,
            outputscale: None,
            outputoffset: None,
            smooth: None,
            snapto: None,
            snapforce: None,
            color: None,
            negativecolor: None,
            ledfill: None,
            output: None,
            button: None,
            selectat: None,
            preset: None,
            loadpreset: None,
            savepreset: None,
            clear: None,
            clearall: None,
            dontsave: None,
        }
    }

    pub fn with_fader(mut self, fader: impl Into<String>) -> Self {
        self.fader = Some(CVInput(fader.into()));
        self
    }

    // Add other builder methods as needed
}

impl BaseCircuit for MotorFader {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "motorfader"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        if let Some(fader) = &self.fader {
            ini.with_section(section).set("fader", &fader.0);
        }
        if let Some(sharewithnext) = &self.sharewithnext {
            ini.with_section(section).set("sharewithnext", &sharewithnext.0);
        }
        // Add other fields...

        ini_to_string(&ini)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motorfader_creation() {
        let fader = MotorFader::new("fader1")
            .with_fader("1");
        
        assert_eq!(fader.id().to_string(), "fader1");
        assert_eq!(fader.section(), "motorfader");
    }
}
