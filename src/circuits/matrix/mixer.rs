use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::Result;
use crate::types::{GateInput, CVInput};
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// CV mixer circuit configuration
/// See DROID manual page 285
#[derive(Debug, Serialize, Deserialize)]
pub struct Mixer {
    id: CircuitId,
    // Input CVs (1-8)
    input1: Option<CVInput>,
    input2: Option<CVInput>,
    input3: Option<CVInput>,
    input4: Option<CVInput>,
    input5: Option<CVInput>,
    input6: Option<CVInput>,
    input7: Option<CVInput>,
    input8: Option<CVInput>,
    // Mix/max mode
    mixmax: Option<CVInput>,
    // Initial state
    startvalue: Option<CVInput>,
    // Button inputs for rows
    button11: Option<GateInput>,
    button12: Option<GateInput>,
    button13: Option<GateInput>,
    button14: Option<GateInput>,
    button21: Option<GateInput>,
    button22: Option<GateInput>,
    button23: Option<GateInput>,
    button24: Option<GateInput>,
    button31: Option<GateInput>,
    button32: Option<GateInput>,
    button33: Option<GateInput>,
    button34: Option<GateInput>,
    button41: Option<GateInput>,
    button42: Option<GateInput>,
    button43: Option<GateInput>,
    button44: Option<GateInput>,
    // LED outputs
    led11: Option<CVInput>,
    led12: Option<CVInput>,
    led13: Option<CVInput>,
    led14: Option<CVInput>,
    led21: Option<CVInput>,
    led22: Option<CVInput>,
    led23: Option<CVInput>,
    led24: Option<CVInput>,
    led31: Option<CVInput>,
    led32: Option<CVInput>,
    led33: Option<CVInput>,
    led34: Option<CVInput>,
    led41: Option<CVInput>,
    led42: Option<CVInput>,
    led43: Option<CVInput>,
    led44: Option<CVInput>,
    // Outputs
    output: Option<CVInput>,
    maximum: Option<CVInput>,
    minimum: Option<CVInput>,
    average: Option<CVInput>,
}

impl Mixer {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            input1: None,
            input2: None,
            input3: None,
            input4: None,
            input5: None,
            input6: None,
            input7: None,
            input8: None,
            mixmax: None,
            startvalue: None,
            button11: None,
            button12: None,
            button13: None,
            button14: None,
            button21: None,
            button22: None,
            button23: None,
            button24: None,
            button31: None,
            button32: None,
            button33: None,
            button34: None,
            button41: None,
            button42: None,
            button43: None,
            button44: None,
            led11: None,
            led12: None,
            led13: None,
            led14: None,
            led21: None,
            led22: None,
            led23: None,
            led24: None,
            led31: None,
            led32: None,
            led33: None,
            led34: None,
            led41: None,
            led42: None,
            led43: None,
            led44: None,
            output: None,
            maximum: None,
            minimum: None,
            average: None,
        }
    }

    pub fn with_input(&mut self, index: u8, value: impl Into<String>) -> &mut Self {
        match index {
            1 => self.input1 = Some(CVInput(value.into())),
            2 => self.input2 = Some(CVInput(value.into())),
            3 => self.input3 = Some(CVInput(value.into())),
            4 => self.input4 = Some(CVInput(value.into())),
            5 => self.input5 = Some(CVInput(value.into())),
            6 => self.input6 = Some(CVInput(value.into())),
            7 => self.input7 = Some(CVInput(value.into())),
            8 => self.input8 = Some(CVInput(value.into())),
            _ => {}
        }
        self
    }
}

impl BaseCircuit for Mixer {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "mixer"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        // Serialize inputs
        for i in 1..=8 {
            if let Some(input) = self.get_input(i) {
                ini.with_section(section).set(&format!("input{}", i), &input.0);
            }
        }

        // Serialize other fields
        if let Some(mixmax) = &self.mixmax {
            ini.with_section(section).set("mixmax", &mixmax.0);
        }
        if let Some(startvalue) = &self.startvalue {
            ini.with_section(section).set("startvalue", &startvalue.0);
        }

        // Serialize matrix buttons and LEDs
        for i in 1..=4 {
            for j in 1..=4 {
                if let Some(button) = self.get_button(i, j) {
                    ini.with_section(section).set(&format!("button{}{}", i, j), &button.0);
                }
                if let Some(led) = self.get_led(i, j) {
                    ini.with_section(section).set(&format!("led{}{}", i, j), &led.0);
                }
            }
        }

        // Serialize outputs
        if let Some(output) = &self.output {
            ini.with_section(section).set("output", &output.0);
        }
        if let Some(maximum) = &self.maximum {
            ini.with_section(section).set("maximum", &maximum.0);
        }
        if let Some(minimum) = &self.minimum {
            ini.with_section(section).set("minimum", &minimum.0);
        }
        if let Some(average) = &self.average {
            ini.with_section(section).set("average", &average.0);
        }

        ini_to_string(&ini)
    }
}

impl Mixer {
    fn get_input(&self, index: u8) -> Option<&CVInput> {
        match index {
            1 => self.input1.as_ref(),
            2 => self.input2.as_ref(),
            3 => self.input3.as_ref(),
            4 => self.input4.as_ref(),
            5 => self.input5.as_ref(),
            6 => self.input6.as_ref(),
            7 => self.input7.as_ref(),
            8 => self.input8.as_ref(),
            _ => None
        }
    }

    fn get_button(&self, row: u8, col: u8) -> Option<&GateInput> {
        match (row, col) {
            (1, 1) => self.button11.as_ref(),
            (1, 2) => self.button12.as_ref(),
            (1, 3) => self.button13.as_ref(),
            (1, 4) => self.button14.as_ref(),
            (2, 1) => self.button21.as_ref(),
            (2, 2) => self.button22.as_ref(),
            (2, 3) => self.button23.as_ref(),
            (2, 4) => self.button24.as_ref(),
            (3, 1) => self.button31.as_ref(),
            (3, 2) => self.button32.as_ref(),
            (3, 3) => self.button33.as_ref(),
            (3, 4) => self.button34.as_ref(),
            (4, 1) => self.button41.as_ref(),
            (4, 2) => self.button42.as_ref(),
            (4, 3) => self.button43.as_ref(),
            (4, 4) => self.button44.as_ref(),
            _ => None
        }
    }

    fn get_led(&self, row: u8, col: u8) -> Option<&CVInput> {
        match (row, col) {
            (1, 1) => self.led11.as_ref(),
            (1, 2) => self.led12.as_ref(),
            (1, 3) => self.led13.as_ref(),
            (1, 4) => self.led14.as_ref(),
            (2, 1) => self.led21.as_ref(),
            (2, 2) => self.led22.as_ref(),
            (2, 3) => self.led23.as_ref(),
            (2, 4) => self.led24.as_ref(),
            (3, 1) => self.led31.as_ref(),
            (3, 2) => self.led32.as_ref(),
            (3, 3) => self.led33.as_ref(),
            (3, 4) => self.led34.as_ref(),
            (4, 1) => self.led41.as_ref(),
            (4, 2) => self.led42.as_ref(),
            (4, 3) => self.led43.as_ref(),
            (4, 4) => self.led44.as_ref(),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixer_creation() {
        let mut mixer = Mixer::new("mixer1");
        mixer.with_input(1, "1V");
        
        assert_eq!(mixer.id().to_string(), "mixer1");
        assert_eq!(mixer.section(), "mixer");
    }
}
