use crate::{ki, rule, rulekey};

use super::{generate_prefix_from_input, RuleSet};

#[test]
fn can_create_new_ruleset_from_rules() {
    let ruleset = RuleSet::new(
        "Base Ruleset",
        vec![
            rule!(L1 CAPSLOCK => 1),
            rule!(L1 LEFTCTRL, R1 J => 116),
            rule!(L1 LEFTCTRL, R1 K => 115),
        ],
    );

    let first_key = rulekey!(L1 CAPSLOCK);
    assert_eq!(*ruleset.rules.get(&first_key).unwrap(), 1);

    let second_key = rulekey!(L1 LEFTCTRL, R1 J);
    assert_eq!(*ruleset.rules.get(&second_key).unwrap(), 116);

    let third_key = rulekey!(L1 LEFTCTRL, R1 K);
    assert_eq!(*ruleset.rules.get(&third_key).unwrap(), 115);
}

#[test]
fn can_generate_key_prefix_from_rule_input() {
    let rule = rule!(L1 CAPSLOCK => 1);
    let prefix = generate_prefix_from_input(rule.input());
    assert_eq!(prefix.len(), 1);
    assert!(*prefix.get(0).unwrap() == ki!(L1 CAPSLOCK));

    let rule = rule!(L1 CAPSLOCK, R1 Z => 1);
    let prefix = generate_prefix_from_input(rule.input());
    assert_eq!(prefix.len(), 1);
    assert!(*prefix.get(0).unwrap() == ki!(L1 CAPSLOCK));

    let rule = rule!(L1 CAPSLOCK, L1 A, R1 O => 1);
    let prefix = generate_prefix_from_input(rule.input());
    assert_eq!(prefix.len(), 2);
    assert!(*prefix.get(0).unwrap() == ki!(L1 CAPSLOCK));
    assert!(*prefix.get(1).unwrap() == ki!(L1 A));
}

#[test]
fn can_generate_prefix_hash_set_when_creating_ruleset() {
    let ruleset = RuleSet::new(
        "Base Ruleset",
        vec![
            rule!(L1 CAPSLOCK => 1),
            rule!(L1 LEFTCTRL, R1 J => 116),
            rule!(L1 LEFTCTRL, R1 J, R1 K => 1),
            rule!(L1 LEFTCTRL, R1 K => 115),
            rule!(L1 LEFTCTRL, R1 K, R1 J => 1),
        ],
    );
    let prefixes = ruleset.prefixes();
    assert_eq!(prefixes.len(), 3);
    assert!(prefixes.contains(&rulekey!(L1 LEFTCTRL)));
    assert!(prefixes.contains(&rulekey!(L1 LEFTCTRL, R1 J)));
    assert!(prefixes.contains(&rulekey!(L1 LEFTCTRL, R1 K)));
}
