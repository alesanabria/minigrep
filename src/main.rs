use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {

        println!("{}", err);

        process::exit(1);
    });
   
    println!("query {}", config.query);

    println!("filename {}", config.filename);

    if let Err(e) = run(config) {

        println!("run err {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.filename)?;

    println!("content {:?}", content);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {

    fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {

            return Err("not enough args");
        }

        let query = args[1].clone();
    
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
}

