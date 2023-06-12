use std::{error::Error, fs};

pub struct Args<'a>{
    pub search: &'a String,
    pub file: &'a String
}

impl Args<'_>{
    pub fn new(args: &Vec<String>) -> Result<Args, &'static str> {
        if args.len() != 3{
            return Err("2 arguments are exepted 1: word to search, 2: file to look at")
        }
        let search = &args[1];
        let file = &args[2];
        Ok(Args{
            search,
             file
        })
    }

}

pub fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(args.file)?;
    
    println!("Inside this text:\n{}", content);
    Ok(())
}