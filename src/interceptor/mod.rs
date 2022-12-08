pub mod incoming_fragment;
pub mod state;

use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::SystemTime,
};

use evdev::AutoRepeat;

use crate::{
    devices::{self, input::EventKindCheck, output::event_from_code},
    utils,
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

    let mut virtual_device = devices::output::new().unwrap();
    let mut state = State::new();
    let mut count = 0;

    for signal in rx {
        match signal {
            TransmitSignal::Key(device_alias, code, value, timestamp) => {
                let fragment = IncomingFragment::new(&device_alias, code, value, timestamp);

                utils::dev_print::sequence_up_time_print(&state, value);

                state.receive(&fragment);

                // if value == 0 {
                //     let event_down = event_from_code(code, 1);
                //     let event_up = event_from_code(code, 0);
                //     virtual_device.emit(&[event_down, event_up]).unwrap();
                // }

                utils::dev_print::sequence_hold_time_print(&state, value, timestamp, &mut count);

                utils::dev_print::sequence_print(&state, code, value);
                utils::dev_print::dev_clear(&fragment);
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

    let auto_repeat_settings = AutoRepeat {
        delay: 150,
        period: 1,
    };
    d.update_auto_repeat(&auto_repeat_settings).unwrap();

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
