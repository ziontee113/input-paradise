use super::incoming_fragment::IncomingFragment;
use chrono::{DateTime, Local};

pub fn dev_print(fragment: &IncomingFragment) {
    let datetime: DateTime<Local> = fragment.timestamp().into();

    println!(
        "{}|{} {} at {}",
        fragment.device_alias(),
        fragment.code(),
        fragment.value(),
        datetime.format("%d/%m/%Y %T %.3f")
    );
}

pub fn dev_clear(fragment: &IncomingFragment) {
    if fragment.code() == 1 {
        print!("{}[2J", 27 as char);
        println!("----------------------");
    }

    if fragment.code() == 2 && fragment.value() == 1 {
        println!("----------------------");
    }

    assert!(fragment.code() != 15, "program terminated by pressing <BS>");
}
