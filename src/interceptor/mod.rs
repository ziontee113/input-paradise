pub mod incoming_fragment;
mod state;

use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::SystemTime,
};

use crate::{
    devices::{self, input::EventKindCheck},
    utils::{self, dev_print::dev_clear},
};

use self::{incoming_fragment::IncomingFragment, state::State};

enum TransmitSignal {
    Key(String, u16, i32, SystemTime),
}

pub fn start() {
    let alias_map = utils::mock_device_alias();

    // ----------------------------------------------------------------

    let (tx, rx) = mpsc::channel();

    for (device_alias, device_path) in alias_map {
        intercept(tx.clone(), device_alias, device_path);
    }

    // ----------------------------------------------------------------

    // let mut virtual_device = devices::output::new().unwrap();
    let mut state = State::new();

    for signal in rx {
        match signal {
            TransmitSignal::Key(device_alias, code, value, timestamp) => {
                let fragment = IncomingFragment::new(&device_alias, code, value, timestamp);
                state.receive(&fragment);

                let result = state
                    .sequence()
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>();
                println!("{:?}", result);

                dev_clear(&fragment);
            }
        }
    }
}

fn intercept(rx: Sender<TransmitSignal>, device_alias: &str, device_path: &str) {
    let device_alias = device_alias.to_string();

    let mut d = devices::input::from_path(device_path);
    match d.grab() {
        Ok(_) => println!("Grabbed {} {} SUCCESSFULLY", device_alias, device_path),
        Err(err) => {
            println!(
                "FAILED TO GRAB {} {},\n{},\n------------------",
                device_alias, device_path, err
            );
        }
    }

    thread::spawn(move || loop {
        match d.fetch_events() {
            Err(err) => println!("Error fetching events. {}", err),
            Ok(events) => {
                for ev in events {
                    if ev.is_type_key() {
                        rx.send(TransmitSignal::Key(
                            device_alias.to_string(),
                            ev.code(),
                            ev.value(),
                            ev.timestamp(),
                        ))
                        .unwrap();
                    }
                }
            }
        }
    });
}
