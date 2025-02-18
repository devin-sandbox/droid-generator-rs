use droid_generator_rs::{Patch, DeviceType, LFO};

#[test]
fn test_duplicate_sections() {
    let mut patch = Patch::new(vec![DeviceType::P2B8]);
    
    let lfo1 = LFO::new("lfo1")
        .with_rate("1.0")
        .with_output("O1");
    
    let lfo2 = LFO::new("lfo2")
        .with_rate("2.0")
        .with_output("O2");
    
    patch.add_circuit(lfo1);
    patch.add_circuit(lfo2);
    
    let output = patch.to_string().unwrap();
    println!("{}", output);
    
    // Verify both LFO sections are present
    assert!(output.contains("rate=1.0"));
    assert!(output.contains("rate=2.0"));
    assert!(output.contains("output=O1"));
    assert!(output.contains("output=O2"));
}
