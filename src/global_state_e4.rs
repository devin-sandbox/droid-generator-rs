use crate::{LFO, MotorFader, Button, Patch};
use crate::error::{DroidError, Result};


/// Maximum number of LFOs allowed in a patch
pub const MAX_ALLOWED_LFOS: u8 = 8;

/// Special value for LFO selection
pub const LFO_SELECT: &str = "_LFO_SELECT";

/// Validates the number of LFOs in a patch
/// 
/// # Arguments
/// * `num` - Number of LFOs to validate
/// 
/// # Returns
/// * `Ok(num)` if the number is valid
/// * `Err` with a descriptive message if invalid
/// 
/// # Examples
/// ```
/// use droid_generator_rs::validate_num_lfos;
/// 
/// assert!(validate_num_lfos(1).is_ok());
/// assert!(validate_num_lfos(8).is_ok());
/// assert!(validate_num_lfos(0).is_err());
/// assert!(validate_num_lfos(9).is_err());
/// ```
pub fn validate_num_lfos(num: u8) -> Result<u8> {
    if num < 1 || num > MAX_ALLOWED_LFOS {
        return Err(DroidError::ValidationError(
            format!("Number of LFOs must be between 1 and {}", MAX_ALLOWED_LFOS)
        ));
    }
    Ok(num)
}

/// Generates an E4 patch with the specified number of LFOs
pub fn generate_patch(num_lfos: u8) -> Result<String> {
    let mut patch = Patch::new(vec![]);  // Use default device list
    let _num_lfos = validate_num_lfos(num_lfos)?;

    // Add main LFO
    let lfo = LFO::new("lfo1")
        .with_sawtooth("O1")
        .with_level("P3.2")
        .with_hz("P3.1 * 100");
    patch.add_circuit(lfo);

    // Add save button
    let save_button = Button::new("button1")
        .with_shortpress("_SAVE")
        .with_button("B1.2");
    patch.add_circuit(save_button);

    // Add load button
    let load_button = Button::new("button2")
        .with_shortpress("_LOAD")
        .with_button("B1.1");
    patch.add_circuit(load_button);

    // Add motor fader
    let fader = MotorFader::new("fader1")
        .with_savepreset("_SAVE")
        .with_fader("1")
        .with_loadpreset("_LOAD");
    patch.add_circuit(fader);

    patch.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_num_lfos() {
        assert!(validate_num_lfos(1).is_ok());
        assert!(validate_num_lfos(8).is_ok());
        assert!(validate_num_lfos(0).is_err());
        assert!(validate_num_lfos(9).is_err());
    }

    #[test]
    fn test_generate_patch() {
        let patch = generate_patch(1).unwrap();
        assert!(patch.contains("[E4]"));
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
}
