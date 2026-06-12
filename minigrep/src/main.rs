use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

fn main() {
    //1. collect arguments
    let args: Vec<String> = env::args().collect(); 
    
    //2. Build Configs
    //we are more interested in extracting the success - so unwrap or else made sense
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
        
    //3. Run Search
    //here we are more interested in dealing with error if it occurs, because if its success then - the method returns () emppty what to do with it
    //hence if let is good here
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in result {
        println!("{line}");
    }

    Ok(())
}

