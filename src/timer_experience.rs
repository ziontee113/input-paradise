use std::{thread, time::Duration};

use tokio::task;

pub fn timer_experience() {
    task::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(50));
        println!("hello world");
    });

    println!("hello venus");
}
