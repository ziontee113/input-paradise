#![warn(clippy::pedantic)]
#![allow(dead_code)]

mod devices;
mod interceptor;
mod rule_library;
mod timer_experience;
mod units;
mod utils;

#[tokio::main()]
async fn main() {
    interceptor::start();

    // timer_experience::timer_experience();
}
