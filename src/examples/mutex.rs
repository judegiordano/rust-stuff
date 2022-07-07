use std::sync::{Arc, Mutex};
use std::thread;

pub fn example() {
    let m = Mutex::new(5);
    {
        // scope closure do drop is called and data can be read
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("{:#?}", m);

    mutex_threading();
}

pub fn mutex_threading() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let val = *counter.lock().unwrap();
    println!("counter value {:#?}", val);
}
