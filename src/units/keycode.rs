use evdev::{EnumParseError, Key};
use std::str::FromStr;

pub fn key_code_from_str(key: &str) -> Result<u16, EnumParseError> {
    let mut final_key = key.to_uppercase();

    if !final_key.contains("BTN_") {
        final_key = format!("KEY_{}", final_key);
    }

    Ok(Key::from_str(&final_key)?.code())
}
