use droid_generator_rs::{
    circuits::BaseCircuit,
    LFO, MotorFader, Case, Mixer, VCO, Sequencer, ClockTool,
    validation::{validate_cv_range, validate_gate}
};

#[test]
fn test_lfo_validation() {
    let lfo = LFO::new("lfo1")
        .with_rate("1.0")
        .with_hz("440");
    
    assert!(validate_cv_range("1.0", -5.0, 5.0, "rate").is_ok());
    assert!(validate_cv_range("440", 0.1, 1000.0, "hz").is_ok());
    
    let ini = lfo.to_ini().unwrap();
    assert!(ini.contains("rate=1.0"));
    assert!(ini.contains("hz=440"));
}

#[test]
fn test_motorfader_validation() {
    let fader = MotorFader::new("fader1")
        .with_fader("1");
    
    assert!(validate_cv_range("1", 1.0, 8.0, "fader").is_ok());
    assert!(validate_cv_range("9", 1.0, 8.0, "fader").is_err());
    
    let ini = fader.to_ini().unwrap();
    assert!(ini.contains("fader=1"));
}

#[test]
fn test_case_validation() {
    let mut case = Case::new("case1");
    case.with_case(1, "1V")
        .with_value(1, "5V");
    
    assert!(validate_cv_range("1V", -5.0, 5.0, "case1").is_ok());
    assert!(validate_cv_range("5V", -5.0, 5.0, "value1").is_ok());
    
    let ini = case.to_ini().unwrap();
    assert!(ini.contains("case1=1V"));
    assert!(ini.contains("value1=5V"));
}

#[test]
fn test_mixer_validation() {
    let mut mixer = Mixer::new("mixer1");
    mixer.with_input(1, "1V");
    
    assert!(validate_cv_range("1V", -5.0, 5.0, "input1").is_ok());
    
    let ini = mixer.to_ini().unwrap();
    assert!(ini.contains("input1=1V"));
}

#[test]
fn test_vco_validation() {
    let vco = VCO::new("vco1")
        .with_pitch("1V")
        .with_hz("440");
    
    assert!(validate_cv_range("1V", -5.0, 5.0, "pitch").is_ok());
    assert!(validate_cv_range("440", 0.1, 20000.0, "hz").is_ok());
    
    let ini = vco.to_ini().unwrap();
    assert!(ini.contains("pitch=1V"));
    assert!(ini.contains("hz=440"));
}

#[test]
fn test_sequencer_validation() {
    let mut seq = Sequencer::new("seq1");
    seq.with_value(1, "1V")
       .with_gate(1, "1");
    
    assert!(validate_cv_range("1V", -5.0, 5.0, "value1").is_ok());
    assert!(validate_gate("1", "gate1").is_ok());
    
    let ini = seq.to_ini().unwrap();
    assert!(ini.contains("value1=1V"));
    assert!(ini.contains("gate1=1"));
}

#[test]
fn test_clocktool_validation() {
    let clock = ClockTool::new("clock1")
        .with_bpm("120")
        .with_multiply("2");
    
    assert!(validate_cv_range("120", 20.0, 300.0, "bpm").is_ok());
    assert!(validate_cv_range("2", 1.0, 16.0, "multiply").is_ok());
    
    let ini = clock.to_ini().unwrap();
    assert!(ini.contains("bpm=120"));
    assert!(ini.contains("multiply=2"));
}
