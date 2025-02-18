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
    let patch = generate_patch(1).unwrap();
    assert!(patch.contains("[p2b8]"));
    assert!(patch.contains("[e4]"));
    assert!(patch.contains("[m4]"));
    assert!(patch.contains("[lfo]"));
    assert!(patch.contains("sawtooth=O1"));
    assert!(patch.contains("level=P3.2"));
    assert!(patch.contains("hz=P3.1 * 100"));
    assert!(patch.contains("[button]"));
    assert!(patch.contains("shortpress=_SAVE"));
    assert!(patch.contains("button=B1.2"));
    assert!(patch.contains("[motorfader]"));
    assert!(patch.contains("fader=1"));
}
