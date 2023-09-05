use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let values = vec!["Hey", "good morning", "from", "new thread"];
        for v in values{
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let values = vec!["Hey", "good evening", "from", "new thread 2"];
        for v in values{
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for receive in rx {
        println!("We received : {}", receive );
    }

    println!("Hello from main");
}