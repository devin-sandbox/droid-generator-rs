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
    // Test with different numLfos values
    for num in 1..=8 {
        let patch = generate_patch(num).unwrap();
        // Verify device sections
        assert!(patch.contains("[p2b8]"));
        assert!(patch.contains("[e4]"));
        assert!(patch.contains("[m4]"));
        // Verify single LFO
        assert!(patch.contains("[lfo]"));
        assert!(patch.contains("sawtooth=O1"));
        assert!(patch.contains("level=P3.2"));
        assert!(patch.contains("hz=P3.1 * 100"));
        // Verify buttons
        assert!(patch.contains("[button]"));
        assert!(patch.contains("shortpress=_SAVE"));
        assert!(patch.contains("button=B1.2"));
        // Verify motorfader
        assert!(patch.contains("[motorfader]"));
        assert!(patch.contains("fader=1"));
        assert!(patch.contains("loadpreset=_LOAD"));
        assert!(patch.contains("savepreset=_SAVE"));
    }
}
