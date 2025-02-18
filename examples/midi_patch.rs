use droid_generator_rs::{Patch, DeviceType, MidiIn};

fn main() {
    let mut patch = Patch::new(vec![DeviceType::P2B8]);
    
    let midi = MidiIn::new("midi1");
    patch.add_circuit(midi);
    
    println!("{}", patch.to_string().unwrap());
}
