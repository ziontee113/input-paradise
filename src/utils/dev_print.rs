use chrono::{DateTime, Local};

use crate::{
    interceptor::{
        incoming_fragment::{IncomingFragment, KeyState},
        state::State,
    },
    utils::code_to_key_name::{self, code_to_key_name},
};

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

pub fn sequence_print(state: &State, code: u16, value: i32) {
    let result = state
        .sequence()
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    print!("{:?}", result);

    if state.sequence().len() > 1 {
        let first_time = state.sequence().get(0).unwrap().timestamp();
        let current_time = state.sequence().last().unwrap().timestamp();
        print!(
            " {}",
            current_time.duration_since(first_time).unwrap().as_millis()
        );
    }

    if value == 0 {
        print!(" ->{}", code_to_key_name(code));
    }

    println!();
}
