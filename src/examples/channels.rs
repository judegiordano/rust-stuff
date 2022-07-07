use std::{sync::mpsc, thread, time::Duration};

pub fn example() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn _send_string() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val: String = String::from("hi");
        tx.send(val).unwrap();
    });

    let received: String = rx.recv().unwrap();
    println!("message: {:#?}", received);
}
