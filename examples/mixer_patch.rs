use droid_generator_rs::{Patch, DeviceType, Mixer, LFO};

fn main() {
    let mut patch = Patch::new(vec![DeviceType::P2B8]);
    
    let mut mixer = Mixer::new("mix1");
    mixer.with_input(1, "O1")
         .with_input(2, "O2");
    
    let lfo1 = LFO::new("lfo1")
        .with_rate("1V")
        .with_output("O1");
    
    let lfo2 = LFO::new("lfo2")
        .with_rate("2V")
        .with_output("O2");
    
    patch.add_circuit(lfo1);
    patch.add_circuit(lfo2);
    patch.add_circuit(mixer);
    
    println!("{}", patch.to_string().unwrap());
}
