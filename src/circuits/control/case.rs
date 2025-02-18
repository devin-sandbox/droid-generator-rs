use crate::circuits::{BaseCircuit, CircuitId};
use crate::error::Result;
use crate::types::CVInput;
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// Case selection circuit configuration
/// See DROID manual page 153
#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    id: CircuitId,
    case1: Option<CVInput>,
    case2: Option<CVInput>,
    case3: Option<CVInput>,
    case4: Option<CVInput>,
    case5: Option<CVInput>,
    case6: Option<CVInput>,
    case7: Option<CVInput>,
    case8: Option<CVInput>,
    case9: Option<CVInput>,
    case10: Option<CVInput>,
    case11: Option<CVInput>,
    case12: Option<CVInput>,
    case13: Option<CVInput>,
    case14: Option<CVInput>,
    case15: Option<CVInput>,
    case16: Option<CVInput>,
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
    else_value: Option<CVInput>,
    output: Option<CVInput>,
}

impl Case {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            case1: None,
            case2: None,
            case3: None,
            case4: None,
            case5: None,
            case6: None,
            case7: None,
            case8: None,
            case9: None,
            case10: None,
            case11: None,
            case12: None,
            case13: None,
            case14: None,
            case15: None,
            case16: None,
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
            else_value: None,
            output: None,
        }
    }

    pub fn with_case(&mut self, index: u8, value: impl Into<String>) -> &mut Self {
        match index {
            1 => self.case1 = Some(CVInput(value.into())),
            2 => self.case2 = Some(CVInput(value.into())),
            // ... implement for all cases
            _ => {}
        }
        self
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
}

impl BaseCircuit for Case {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "case"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        // Serialize all case conditions
        for i in 1..=16 {
            if let Some(case) = self.get_case(i) {
                ini.with_section(section).set(&format!("case{}", i), &case.0);
            }
        }

        // Serialize all case values
        for i in 1..=16 {
            if let Some(value) = self.get_value(i) {
                ini.with_section(section).set(&format!("value{}", i), &value.0);
            }
        }

        if let Some(else_value) = &self.else_value {
            ini.with_section(section).set("else", &else_value.0);
        }

        if let Some(output) = &self.output {
            ini.with_section(section).set("output", &output.0);
        }

        ini_to_string(&ini)
    }
}

impl Case {
    fn get_case(&self, index: u8) -> Option<&CVInput> {
        match index {
            1 => self.case1.as_ref(),
            2 => self.case2.as_ref(),
            // ... implement for all cases
            _ => None
        }
    }

    fn get_value(&self, index: u8) -> Option<&CVInput> {
        match index {
            1 => self.value1.as_ref(),
            2 => self.value2.as_ref(),
            // ... implement for all values
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_creation() {
        let mut case = Case::new("case1");
        case.with_case(1, "1V")
            .with_value(1, "5V");
        
        assert_eq!(case.id().to_string(), "case1");
        assert_eq!(case.section(), "case");
    }
}
