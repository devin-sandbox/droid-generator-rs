use droid_generator_rs::global_state_e4::generate_patch;

fn main() {
    let num_lfos = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);

    match generate_patch(num_lfos) {
        Ok(patch) => println!("{}", patch),
        Err(e) => eprintln!("Error: {}", e),
    }
}
