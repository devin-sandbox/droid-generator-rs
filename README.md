# DROID Generator (Rust)

A Rust implementation of the DROID patch generator, providing a type-safe circuit configuration system for generating INI configuration files for DROID devices.

## Features

- Type-safe circuit configuration
- Comprehensive circuit implementations following DROID manual specifications
- INI file generation for DROID patches
- Support for multiple DROID devices (P2B8, E4, M4)

## Implemented Circuits

- Control: Case
- IO: MotorFader
- Matrix: Mixer
- MIDI: MidiIn
- Modulation: LFO
- Sequencing: Sequencer
- Signal: VCO
- Timing: ClockTool

## Usage

```rust
use droid_generator_rs::{Patch, DeviceType, LFO};

fn main() {
    let mut patch = Patch::new(vec![DeviceType::P2B8]);
    let lfo = LFO::new("lfo1")
        .with_rate("1V")
        .with_output("O1");
    patch.add_circuit(lfo);
    println!("{}", patch.to_string().unwrap());
}
```

## Development

```bash
# Run tests
cargo test

# Run examples
cargo run --example basic_patch
cargo run --example midi_patch
cargo run --example mixer_patch
```

## References

- [DROID Manual](https://dermannmitdermaschine.de/download/droid-manual-blue-6.pdf)

A Rust implementation of the DROID patch generator. This tool generates DROID patches in INI format following the specifications from the DROID manual.

## Features
- Type-safe circuit configurations
- INI format patch generation
- Validation against DROID specifications

## Usage
```rust
use droid_generator_rs::{Patch, DeviceType};

fn main() {
    let mut patch = Patch::new(vec![DeviceType::P2B8]);
    // Add circuits and generate patch
    println!("{}", patch.to_string().unwrap());
}
```

## Development
This project is a Rust rewrite of the TypeScript droid-generator implementation.
