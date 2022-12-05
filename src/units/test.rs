use super::keycode::key_code_from_str;

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
