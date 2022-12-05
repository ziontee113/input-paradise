use super::keycode::code_from_key;

#[test]
fn can_return_u16_keycode_from_str() {
    assert_eq!(code_from_key("ESC").unwrap(), 1);
    assert_eq!(code_from_key("D").unwrap(), 32);
    assert_eq!(code_from_key("j").unwrap(), 36);
    assert_eq!(code_from_key("BTN_START").unwrap(), 0x13b);
    assert_eq!(code_from_key("BTn_sELECT").unwrap(), 0x13a);
}
