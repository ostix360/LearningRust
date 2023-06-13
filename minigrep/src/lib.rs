use std::{error::Error, fs, env};

pub struct Config<'a>{
    pub search: &'a String,
    pub file: &'a String,
    pub sensitive_case: bool
}

impl Config<'_>{
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3{
            return Err("2 arguments are exepted 1: word to search, 2: file to look at")
        }
        let search = &args[1];
        let file = &args[2];

        let sensitive_case = env::var("MINIGREP_INSENSITIVE_CASE").is_err();
        Ok(Config {
            search,
            file,
            sensitive_case,
        })
    }

}


pub fn find_case<'a>(search: &str, content: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();

    for line in content.lines(){
        if line.contains(search){
            result.push(line);
        }
    }

    result
}


pub fn find_uncased<'a>(search: &str, content: &'a str) -> Vec<&'a str>{
    let search = search.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines(){
        if line.to_lowercase().contains(&search){
            result.push(line);
        }
    }

    result
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file)?;

    let res = if config.sensitive_case{
        find_case(&config.search, &content)
    }else{
        find_uncased(&config.search, &content)
    };
    
    for line in res{
        println!("{}", line);
    }
    Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case(){
        let search = "duct";
        let content = "\
Rust:
security, fasted, productivity.
Get this 3 elements at the same time.";

        assert_eq!(
            vec!["security, fasted, productivity."],
            find_case(search, content)
        );
    }

     #[test]
    fn uncased(){
        let search = "dUCt";
        let content = "\
Rust:
security, fasted, productivity.
Get this 3 elements at the same time.";

        assert_eq!(
            vec!["security, fasted, productivity."],
            find_uncased(search, content)
        );
    }
}

