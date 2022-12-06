use std::{sync::mpsc, thread, time::Duration};

use tokio::task;

pub fn timer_experience() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    task::spawn_blocking(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(50));
            tx.send(format!("first loop - {i}")).unwrap();
        }
    });

    task::spawn_blocking(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(50));
            tx2.send(format!("second loop - {i}")).unwrap();
        }
    });

    task::spawn_blocking(move || {
        for msg in rx {
            println!("{msg}");
        }
        println!("finish last");
    });

    println!("finish first");
}
