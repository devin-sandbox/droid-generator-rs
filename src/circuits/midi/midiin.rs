use crate::circuits::{BaseCircuit, CircuitId, MidiChannelCircuit};
use crate::error::Result;
use crate::types::{GateInput, CVInput, EnumInput, BooleanInput};
use crate::utils::ini_to_string;
use ini::Ini;
use serde::{Serialize, Deserialize};

/// MIDI input circuit configuration
/// See DROID manual page 260
#[derive(Debug, Serialize, Deserialize)]
pub struct MidiIn {
    id: CircuitId,
    trs: Option<CVInput>,
    usb: Option<CVInput>,
    initial_running: Option<EnumInput>,
    system_reset: Option<GateInput>,
    channel: Option<CVInput>,
    tuning_mode: Option<BooleanInput>,
    tuning_pitch: Option<CVInput>,
    transpose: Option<CVInput>,
    hold_velocity: Option<BooleanInput>,
    pitch_bend_range: Option<CVInput>,
    bend_pitch: Option<BooleanInput>,
    voice_allocation: Option<EnumInput>,
    round_robin: Option<BooleanInput>,
    note_gap: Option<CVInput>,
    // CC numbers (1-4)
    cc_number1: Option<CVInput>,
    cc_number2: Option<CVInput>,
    cc_number3: Option<CVInput>,
    cc_number4: Option<CVInput>,
    // Note range
    lowest_note: Option<CVInput>,
    highest_note: Option<CVInput>,
    // Individual note triggers (1-16)
    note1: Option<CVInput>,
    note2: Option<CVInput>,
    note3: Option<CVInput>,
    note4: Option<CVInput>,
    note5: Option<CVInput>,
    note6: Option<CVInput>,
    note7: Option<CVInput>,
    note8: Option<CVInput>,
    note9: Option<CVInput>,
    note10: Option<CVInput>,
    note11: Option<CVInput>,
    note12: Option<CVInput>,
    note13: Option<CVInput>,
    note14: Option<CVInput>,
    note15: Option<CVInput>,
    note16: Option<CVInput>,
    // Outputs
    pitch1: Option<CVInput>,
    pitch2: Option<CVInput>,
    pitch3: Option<CVInput>,
    pitch4: Option<CVInput>,
    pitch5: Option<CVInput>,
    pitch6: Option<CVInput>,
    pitch7: Option<CVInput>,
    pitch8: Option<CVInput>,
    // Gates
    gate1: Option<GateInput>,
    gate2: Option<GateInput>,
    gate3: Option<GateInput>,
    gate4: Option<GateInput>,
    gate5: Option<GateInput>,
    gate6: Option<GateInput>,
    gate7: Option<GateInput>,
    gate8: Option<GateInput>,
    // Other outputs
    clock: Option<GateInput>,
    clock8: Option<GateInput>,
    clock8t: Option<GateInput>,
    clock16: Option<GateInput>,
    clock4: Option<GateInput>,
    midi_clock: Option<GateInput>,
    start: Option<GateInput>,
    continue_signal: Option<GateInput>,
    stop: Option<GateInput>,
    running: Option<GateInput>,
    active: Option<GateInput>,
    pitch_bend: Option<CVInput>,
    program_change: Option<GateInput>,
    program: Option<CVInput>,
    bank: Option<CVInput>,
    mod_wheel: Option<CVInput>,
    volume: Option<CVInput>,
    portamento: Option<GateInput>,
    soft: Option<GateInput>,
}

impl MidiIn {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CircuitId::new(id.into()),
            trs: None,
            usb: None,
            initial_running: None,
            system_reset: None,
            channel: None,
            tuning_mode: None,
            tuning_pitch: None,
            transpose: None,
            hold_velocity: None,
            pitch_bend_range: None,
            bend_pitch: None,
            voice_allocation: None,
            round_robin: None,
            note_gap: None,
            cc_number1: None,
            cc_number2: None,
            cc_number3: None,
            cc_number4: None,
            lowest_note: None,
            highest_note: None,
            note1: None,
            note2: None,
            note3: None,
            note4: None,
            note5: None,
            note6: None,
            note7: None,
            note8: None,
            note9: None,
            note10: None,
            note11: None,
            note12: None,
            note13: None,
            note14: None,
            note15: None,
            note16: None,
            pitch1: None,
            pitch2: None,
            pitch3: None,
            pitch4: None,
            pitch5: None,
            pitch6: None,
            pitch7: None,
            pitch8: None,
            gate1: None,
            gate2: None,
            gate3: None,
            gate4: None,
            gate5: None,
            gate6: None,
            gate7: None,
            gate8: None,
            clock: None,
            clock8: None,
            clock8t: None,
            clock16: None,
            clock4: None,
            midi_clock: None,
            start: None,
            continue_signal: None,
            stop: None,
            running: None,
            active: None,
            pitch_bend: None,
            program_change: None,
            program: None,
            bank: None,
            mod_wheel: None,
            volume: None,
            portamento: None,
            soft: None,
        }
    }

    // Add builder methods for configuration
}

impl BaseCircuit for MidiIn {
    fn id(&self) -> &CircuitId {
        &self.id
    }

    fn section(&self) -> &str {
        "midiin"
    }

    fn to_ini(&self) -> Result<String> {
        let mut ini = Ini::new();
        let section = Some(self.section());

        if let Some(channel) = &self.channel {
            ini.with_section(section).set("channel", &channel.0);
        }
        if let Some(note1) = &self.note1 {
            ini.with_section(section).set("note1", &note1.0);
        }
        if let Some(note2) = &self.note2 {
            ini.with_section(section).set("note2", &note2.0);
        }
        if let Some(gate1) = &self.gate1 {
            ini.with_section(section).set("gate1", &gate1.0);
        }
        if let Some(gate2) = &self.gate2 {
            ini.with_section(section).set("gate2", &gate2.0);
        }
        if let Some(pitch_bend) = &self.pitch_bend {
            ini.with_section(section).set("pitchbend", &pitch_bend.0);
        }
        if let Some(mod_wheel) = &self.mod_wheel {
            ini.with_section(section).set("modwheel", &mod_wheel.0);
        }
        if let Some(cc_number1) = &self.cc_number1 {
            ini.with_section(section).set("ccnumber1", &cc_number1.0);
        }
        if let Some(cc_number2) = &self.cc_number2 {
            ini.with_section(section).set("ccnumber2", &cc_number2.0);
        }

        ini_to_string(&ini)
    }
}

impl MidiChannelCircuit for MidiIn {
    fn channel(&self) -> Option<&str> {
        self.channel.as_ref().map(|c| c.0.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midiin_creation() {
        let midi = MidiIn::new("midi1");
        assert_eq!(midi.id().to_string(), "midi1");
        assert_eq!(midi.section(), "midiin");
    }
}
