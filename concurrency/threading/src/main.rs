use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![0,1,2];
    let handler = thread::spawn(move || {
        // v is now here
        for i in v{
            println!("Hello n° {} from spawn", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // v is out of scope here

    for i in 1..6 {
        println!("Hello n°{} from main", i);
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();
}
