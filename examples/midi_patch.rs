use droid_generator_rs::{Patch, DeviceType, MidiIn, VCO};

fn main() {
    let mut patch = Patch::new(vec![DeviceType::P2B8]);
    
    let midi = MidiIn::new("midi1");
    let vco = VCO::new("vco1")
        .with_pitch("1V")
        .with_hz("440");
    
    patch.add_circuit(midi);
    patch.add_circuit(vco);
    
    println!("{}", patch.to_string().unwrap());
}
