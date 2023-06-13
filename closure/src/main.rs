use std::thread;
use std::time::Duration;

fn main() {
    let user_intensify = 10;
    let rand = 7;

    generate_exercise(user_intensify, rand);
}

fn big_calc(intensity: u32) -> u32 {
    println!("Heavy calc");
    thread::sleep(Duration::from_millis(2000));
    intensity
}


fn generate_exercise(intensity: u32, rand: u32) {
    if intensity < 25 {
        println!("{} pompes", big_calc(intensity));
        println!("puis {} abdominaux", big_calc(intensity));
    }else{
        if rand == 3 {
            println!("take a break today!");
        }else{
            println!("Run during {} mins", big_calc(intensity));
        }
    }

}
