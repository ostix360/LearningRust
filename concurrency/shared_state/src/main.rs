use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut nb = counter.lock().unwrap();

            *nb += 1
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }

    println!("Result : {}", * counter.lock().unwrap());
}
