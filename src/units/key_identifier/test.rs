use crate::ki;

#[test]
fn can_use_macro_to_create_key_identifier() {
    let key = ki!(L1 D);
    assert_eq!(key.to_string(), "L1|D");

    let key = ki!(R2 Z);
    assert_eq!(key.to_string(), "R2|Z");
}
