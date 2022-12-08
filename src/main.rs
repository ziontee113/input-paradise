#![warn(clippy::pedantic)]
#![allow(dead_code)]

mod devices;
mod interceptor;
mod rule_library;
mod units;
mod utils;

fn main() {
    devices::input::print_paths();

    // interceptor::start();
}
