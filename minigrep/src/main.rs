use std::{env, process};

use minigrep::Args;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("Error with arguments:\n{}", err);
        process::exit(1);
    });
    
    println!("We are looking for {:?} in {:?}", args.search, args.file);

    if let Err(e) = minigrep::run(args){
        println!("Error :\n{}", e);

        process::exit(1);
    }

}
