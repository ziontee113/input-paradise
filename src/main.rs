#![warn(clippy::pedantic)]
#![allow(dead_code)]

mod devices;
mod interceptor;
mod utils;

fn main() {
    interceptor::start();
}
