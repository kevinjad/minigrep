use std::{env, fs};
use std::error::Error;

pub struct Config{
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("insufficient argument")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_insensitive = env::var("CASE_INSENSITIVE").unwrap_or_else(|_|{String::from("0")}); 
        let case_insensitive: bool = case_insensitive == "1";
        Ok(Config {
            query,
            filename,
            case_sensitive: !case_insensitive,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
                result.push(line)
             }     
    }
    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename).expect("problem reading file or File does not exist");
    
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}",line);
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    
   use super::*;

   #[test] 
   fn one_result(){
        let query = "duct";
        let contents = "Rust: safe, fast, productive.\nPick three";

        assert_eq!(vec!["Rust: safe, fast, productive."], search(query, contents));
   } 

   #[test]
   fn case_insensitive(){
       let query = "who";
       let contents = "I'm nobody! Who are you?\nAre you nobody, too?";

       assert_eq!(vec!["I'm nobody! Who are you?"], search_case_insensitive(query, contents));
   }
}
