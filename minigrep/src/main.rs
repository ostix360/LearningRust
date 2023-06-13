use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error with arguments:\n{}", err);
        process::exit(1);
    });


    if let Err(e) = minigrep::run(args){
        eprintln!("Error :\n{}", e);

        process::exit(1);
    }

}
