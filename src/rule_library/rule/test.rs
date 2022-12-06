use crate::{rule, rulekey};

#[test]
fn can_create_new_rule_with_multiple_inputs_and_single_output_using_macro() {
    let rule = rule!(L1 LEFTCTRL, R1 J => 115);

    assert_eq!(rule.input.get(0).unwrap().device_alias(), "L1");
    assert_eq!(rule.input.get(0).unwrap().code(), 29);
    assert_eq!(rule.input.get(1).unwrap().device_alias(), "R1");
    assert_eq!(rule.input.get(1).unwrap().code(), 36);
    assert_eq!(rule.output, 115);
}

#[test]
fn can_create_vector_of_key_identifiers_with_macro() {
    let vec = rulekey!(L1 LEFTCTRL, R1 J);
    assert_eq!(vec.len(), 2);

    let vec = rulekey!(L1 K);
    assert_eq!(vec.len(), 1);
}
