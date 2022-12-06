use chrono::{DateTime, Local};

use crate::interceptor::incoming_fragment::{IncomingFragment, KeyState};

pub fn dev_print(fragment: &IncomingFragment) {
    let datetime: DateTime<Local> = fragment.timestamp().into();

    println!(
        "{}|{} {:?} at {}",
        fragment.key().device_alias(),
        fragment.key().code(),
        fragment.value(),
        datetime.format("%d/%m/%Y %T %.3f")
    );
}

pub fn dev_clear(fragment: &IncomingFragment) {
    if fragment.key().code() == 1 {
        print!("{}[2J", 27 as char);
        println!("----------------------");
    }

    if fragment.key().code() == 2 && fragment.value() == KeyState::Down {
        println!("----------------------");
    }

    assert!(
        fragment.key().code() != 15,
        "program terminated by pressing <BS>"
    );
}
