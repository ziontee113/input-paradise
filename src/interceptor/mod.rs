mod state;
mod incoming_fragment;

use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::{Duration, SystemTime},
};

use crate::{
    devices::{self, input::EventKindCheck, output::event_from_code},
    utils,
};

enum TransmitSignal {
    Key(String, u16, i32, SystemTime),
    Dispatch(u16, i32),

    Nayeon(String),
}

struct DispatchState {
    pending: bool,
    code: u16,
    value: i32,
}
type ArcDisplatchState = Arc<Mutex<DispatchState>>;

pub fn start() {
    let alias_map = utils::mock_device_alias();

    // ----------------------------------------------------------------

    let (tx, rx) = mpsc::channel();

    for (device_alias, device_path) in alias_map {
        intercept(tx.clone(), device_alias, device_path);
    }

    // ----------------------------------------------------------------

    let (nayeon_tx, nayeon_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    thread::spawn(move || {
        for signal in nayeon_rx {
            println!("{signal}");
        }
    });

    // ----------------------------------------------------------------

    let (main_to_dispatch_tx, main_to_dispatch_rx): (
        Sender<ArcDisplatchState>,
        Receiver<ArcDisplatchState>,
    ) = mpsc::channel();

    thread::spawn(move || {
        let interval_limit = 20;

        for dispatch_state in main_to_dispatch_rx {
            let mut dispatch_state = dispatch_state.lock().unwrap();

            if !dispatch_state.pending {
                dispatch_state.pending = true;
                thread::sleep(Duration::from_millis(interval_limit));

                tx.send(TransmitSignal::Dispatch(
                    dispatch_state.code,
                    dispatch_state.value,
                ))
                .unwrap();
            }
        }
    });

    // ----------------------------------------------------------------

    let mut virtual_device = devices::output::new().unwrap();
    let arc_dispatch_state = Arc::new(Mutex::new(DispatchState {
        pending: false,
        code: 0,
        value: 0,
    }));

    for signal in rx {
        match signal {
            TransmitSignal::Nayeon(nayeon_signal) => {
                println!("Nayeon is {nayeon_signal}");
            }
            TransmitSignal::Dispatch(code, value) => {
                let mut dispatch_state = arc_dispatch_state.lock().unwrap();
                dispatch_state.pending = false;

                // println!("{code}, {value}");

                let event = event_from_code(code, value);
                virtual_device.emit(&[event]).unwrap();
            }
            TransmitSignal::Key(_device_alias, code, value, _timestamp) => {
                if value == 0 {
                    // println!("{device_alias}, {code}, {value}");

                    let event = event_from_code(code, value);
                    virtual_device.emit(&[event]).unwrap();
                } else {
                    let mut dispatch_state = arc_dispatch_state.lock().unwrap();

                    if dispatch_state.pending {
                        nayeon_tx.send("Im Nayeon!!!".to_string()).unwrap();
                    } else {
                        dispatch_state.code = code;
                        dispatch_state.value = value;

                        main_to_dispatch_tx
                            .send(Arc::clone(&arc_dispatch_state))
                            .unwrap();
                    }
                }
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
