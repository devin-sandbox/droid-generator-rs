use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::Result;
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

    pub fn with_savepreset(mut self, savepreset: impl Into<String>) -> Self {
        self.savepreset = Some(GateInput(savepreset.into()));
        self
    }

    pub fn with_loadpreset(mut self, loadpreset: impl Into<String>) -> Self {
        self.loadpreset = Some(GateInput(loadpreset.into()));
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

        // Fields in order matching TypeScript interface
        if let Some(fader) = &self.fader {
            ini.with_section(section).set("fader", &fader.0);
        }
        if let Some(sharewithnext) = &self.sharewithnext {
            ini.with_section(section).set("sharewithnext", &sharewithnext.0);
        }
        if let Some(ledvalue) = &self.ledvalue {
            ini.with_section(section).set("ledvalue", &ledvalue.0);
        }
        if let Some(ledcolor) = &self.ledcolor {
            ini.with_section(section).set("ledcolor", &ledcolor.0);
        }
        if let Some(startvalue) = &self.startvalue {
            ini.with_section(section).set("startvalue", &startvalue.0);
        }
        if let Some(notches) = &self.notches {
            ini.with_section(section).set("notches", &notches.0);
        }
        if let Some(outputscale) = &self.outputscale {
            ini.with_section(section).set("outputscale", &outputscale.0);
        }
        if let Some(outputoffset) = &self.outputoffset {
            ini.with_section(section).set("outputoffset", &outputoffset.0);
        }
        if let Some(smooth) = &self.smooth {
            ini.with_section(section).set("smooth", &smooth.0);
        }
        if let Some(snapto) = &self.snapto {
            ini.with_section(section).set("snapto", &snapto.0);
        }
        if let Some(snapforce) = &self.snapforce {
            ini.with_section(section).set("snapforce", &snapforce.0);
        }
        if let Some(color) = &self.color {
            ini.with_section(section).set("color", &color.0);
        }
        if let Some(negativecolor) = &self.negativecolor {
            ini.with_section(section).set("negativecolor", &negativecolor.0);
        }
        if let Some(ledfill) = &self.ledfill {
            ini.with_section(section).set("ledfill", &ledfill.0);
        }
        if let Some(output) = &self.output {
            ini.with_section(section).set("output", &output.0);
        }
        if let Some(button) = &self.button {
            ini.with_section(section).set("button", &button.0);
        }
        if let Some(selectat) = &self.selectat {
            ini.with_section(section).set("selectat", &selectat.0);
        }
        if let Some(preset) = &self.preset {
            ini.with_section(section).set("preset", &preset.0);
        }
        if let Some(loadpreset) = &self.loadpreset {
            ini.with_section(section).set("loadpreset", &loadpreset.0);
        }
        if let Some(savepreset) = &self.savepreset {
            ini.with_section(section).set("savepreset", &savepreset.0);
        }
        if let Some(clear) = &self.clear {
            ini.with_section(section).set("clear", &clear.0);
        }
        if let Some(clearall) = &self.clearall {
            ini.with_section(section).set("clearall", &clearall.0);
        }
        if let Some(dontsave) = &self.dontsave {
            ini.with_section(section).set("dontsave", &dontsave.0);
        }

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

    #[test]
    fn test_motorfader_preset_management() {
        let fader = MotorFader::new("fader1")
            .with_fader("1")
            .with_savepreset("_SAVE")
            .with_loadpreset("_LOAD");
        
        let ini = fader.to_ini().unwrap();
        assert!(ini.contains("fader=1"));
        assert!(ini.contains("savepreset=_SAVE"));
        assert!(ini.contains("loadpreset=_LOAD"));
    }
}
