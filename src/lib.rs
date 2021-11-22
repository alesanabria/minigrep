use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &content) {
        println!("{}", line)
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {

            return Err("not enough args");
        }

        let query = args[1].clone();
    
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
 
    let mut results = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {

        if line.to_lowercase().contains(&query) {

            results.push(line);
        }
    }
    
    return results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_one_result() {

        let query = "test";

        let contents = "rust has \nin build \ntesting \ntest";

        assert_eq!(vec!("testing ", "test"), search(query, contents));
    }
}