use droid_generator_rs::{
    Patch, DeviceType, LFO, MotorFader
};

#[test]
fn test_patch_creation() {
    let patch = Patch::new(vec![DeviceType::P2B8]);
    let ini = patch.to_string().unwrap();
    assert!(ini.contains("[P2B8]"));
}

#[test]
fn test_lfo_in_patch() {
    let mut patch = Patch::new(vec![DeviceType::P2B8]);
    let lfo = LFO::new("lfo1")
        .with_rate("1.0")
        .with_hz("440");
    patch.add_circuit(lfo);
    
    let ini = patch.to_string().unwrap();
    assert!(ini.contains("[P2B8]"));
    assert!(ini.contains("[lfo]"));
    assert!(ini.contains("rate=1.0"));
    assert!(ini.contains("hz=440"));
}

#[test]
fn test_motorfader_in_patch() {
    let mut patch = Patch::new(vec![DeviceType::M4]);
    let fader = MotorFader::new("fader1")
        .with_fader("1");
    patch.add_circuit(fader);
    
    let ini = patch.to_string().unwrap();
    assert!(ini.contains("[M4]"));
    assert!(ini.contains("[motorfader]"));
    assert!(ini.contains("fader=1"));
}

#[test]
fn test_multiple_devices() {
    let mut patch = Patch::new(vec![DeviceType::P2B8, DeviceType::M4]);
    let lfo = LFO::new("lfo1").with_rate("1.0");
    let fader = MotorFader::new("fader1").with_fader("1");
    
    patch.add_circuit(lfo);
    patch.add_circuit(fader);
    
    let ini = patch.to_string().unwrap();
    assert!(ini.contains("[P2B8]"));
    assert!(ini.contains("[M4]"));
    assert!(ini.contains("[lfo]"));
    assert!(ini.contains("[motorfader]"));
}
