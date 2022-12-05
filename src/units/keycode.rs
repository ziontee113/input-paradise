use evdev::{EnumParseError, Key};
use std::str::FromStr;

pub fn key_code_from_str(key: &str) -> Result<u16, EnumParseError> {
    let mut final_key = key.to_uppercase();

    if !final_key.contains("BTN_") {
        final_key = format!("KEY_{}", final_key);
    }

    Ok(Key::from_str(&final_key)?.code())
}

#[cfg(test)]
mod keycode_test {
    use super::*;

    #[test]
    fn can_return_u16_keycode_from_str() {
        assert_eq!(key_code_from_str("ESC").unwrap(), 1);
        assert_eq!(key_code_from_str("D").unwrap(), 32);
        assert_eq!(key_code_from_str("j").unwrap(), 36);
        assert_eq!(key_code_from_str("leftctrl").unwrap(), 29);
        assert_eq!(key_code_from_str("BTN_START").unwrap(), 0x13b);
        assert_eq!(key_code_from_str("BTn_sELECT").unwrap(), 0x13a);

        assert!(key_code_from_str("escape").is_err());
        assert!(key_code_from_str("left control").is_err());
        assert!(key_code_from_str("left_control").is_err());
    }
}
