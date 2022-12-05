#![warn(clippy::pedantic)]
#![allow(dead_code)]

mod devices;
mod interceptor;
mod units;
mod utils;

fn main() {
    interceptor::start();
}
