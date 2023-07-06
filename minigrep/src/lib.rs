use std::{env, error::Error, fs};

pub struct Config {
    pub search: String,
    pub file: String,
    pub sensitive_case: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let search = match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot find the String to search"),
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot the file name"),
        };

        let sensitive_case = env::var("MINIGREP_INSENSITIVE_CASE").is_err();
        Ok(Config {
            search,
            file,
            sensitive_case,
        })
    }
}

pub fn find_case<'a>(search: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(search))
        .collect()
}

pub fn find_uncased<'a>(search: &str, content: &'a str) -> Vec<&'a str> {
    let search = search.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&search))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file)?;

    let res = if config.sensitive_case {
        find_case(&config.search, &content)
    } else {
        find_uncased(&config.search, &content)
    };

    for line in res {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
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
    fn uncased() {
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
