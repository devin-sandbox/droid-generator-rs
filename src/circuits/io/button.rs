use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::Result;
use crate::types::{GateInput, CVInput};
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// Button circuit configuration
/// See DROID manual page 141
#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    id: CircuitId,
    button: Option<GateInput>,
    shortpress: Option<GateInput>,
    longpress: Option<GateInput>,
    led: Option<GateInput>,
    offvalue: Option<CVInput>,
    onvalue: Option<CVInput>,
    value1: Option<CVInput>,
    value2: Option<CVInput>,
    value3: Option<CVInput>,
    value4: Option<CVInput>,
    states: Option<CVInput>,
    startvalue: Option<CVInput>,
    output: Option<CVInput>,
    inverted: Option<CVInput>,
    negated: Option<CVInput>,
    savepreset: Option<GateInput>,
    loadpreset: Option<GateInput>,
}

impl Button {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            button: None,
            shortpress: None,
            longpress: None,
            led: None,
            offvalue: None,
            onvalue: None,
            value1: None,
            value2: None,
            value3: None,
            value4: None,
            states: None,
            startvalue: None,
            output: None,
            inverted: None,
            negated: None,
            savepreset: None,
            loadpreset: None,
        }
    }

    pub fn with_button(mut self, button: impl Into<String>) -> Self {
        self.button = Some(GateInput(button.into()));
        self
    }

    pub fn with_shortpress(mut self, shortpress: impl Into<String>) -> Self {
        self.shortpress = Some(GateInput(shortpress.into()));
        self
    }

    pub fn with_longpress(mut self, longpress: impl Into<String>) -> Self {
        self.longpress = Some(GateInput(longpress.into()));
        self
    }

    pub fn with_led(mut self, led: impl Into<String>) -> Self {
        self.led = Some(GateInput(led.into()));
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
}

impl BaseCircuit for Button {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "button"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        if let Some(button) = &self.button {
            ini.with_section(section).set("button", &button.0);
        }
        if let Some(shortpress) = &self.shortpress {
            ini.with_section(section).set("shortpress", &shortpress.0);
        }
        if let Some(longpress) = &self.longpress {
            ini.with_section(section).set("longpress", &longpress.0);
        }
        if let Some(led) = &self.led {
            ini.with_section(section).set("led", &led.0);
        }
        if let Some(savepreset) = &self.savepreset {
            ini.with_section(section).set("savepreset", &savepreset.0);
        }
        if let Some(loadpreset) = &self.loadpreset {
            ini.with_section(section).set("loadpreset", &loadpreset.0);
        }

        ini_to_string(&ini)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new("button1")
            .with_button("B1.1")
            .with_shortpress("_SAVE");
        
        assert_eq!(button.id().to_string(), "button1");
        assert_eq!(button.section(), "button");
        
        let ini = button.to_ini().unwrap();
        assert!(ini.contains("button=B1.1"));
        assert!(ini.contains("shortpress=_SAVE"));
    }

    #[test]
    fn test_button_preset_management() {
        let button = Button::new("button1")
            .with_button("B1.1")
            .with_savepreset("_SAVE")
            .with_loadpreset("_LOAD");
        
        let ini = button.to_ini().unwrap();
        assert!(ini.contains("savepreset=_SAVE"));
        assert!(ini.contains("loadpreset=_LOAD"));
    }
}
