use droid_generator_rs::{Patch, DeviceType, LFO};

fn main() {
    let mut patch = Patch::new(vec![DeviceType::P2B8]);
    let lfo = LFO::new("lfo1")
        .with_rate("1V")
        .with_output("O1");
    patch.add_circuit(lfo);
    println!("{}", patch.to_string().unwrap());
}
