use droid_generator_rs::global_state_e4::{generate_patch, validate_num_lfos, MAX_ALLOWED_LFOS};

#[test]
fn test_validate_num_lfos() {
    assert!(validate_num_lfos(1).is_ok());
    assert!(validate_num_lfos(MAX_ALLOWED_LFOS).is_ok());
    assert!(validate_num_lfos(0).is_err());
    assert!(validate_num_lfos(MAX_ALLOWED_LFOS + 1).is_err());
}

#[test]
fn test_generate_patch() {
    let expected_output = "\
# LABELS: master=18
[p2b8]
[e4]
[m4]

[lfo]
sawtooth=O1
level=P3.2
hz=P3.1 * 100

[button]
shortpress=_SAVE
button=B1.2

[button]
shortpress=_LOAD
button=B1.1

[motorfader]
savepreset=_SAVE
fader=1
loadpreset=_LOAD
";

    // Test with different numLfos values (1-8)
    for num in 1..=8 {
        let patch = generate_patch(num).unwrap();
        assert_eq!(patch, expected_output);
    }
}

#[test]
fn test_invalid_lfo_count() {
    assert!(generate_patch(0).is_err());
    assert!(generate_patch(9).is_err());
}
