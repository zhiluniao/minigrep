#[macro_use]
extern crate lazy_static;

use std::env;
use std::error::Error;
use std::fs;
use std::result::Result;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

lazy_static! {
    static ref SENSITIVE_WORDS_SET: HashSet<&'static str> = {
        let mut set = HashSet::<&str>::new();
        set.insert("demo");
        println!("{:?}",set);
        set
    };
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("config.case_sensitive:{}", config.case_sensitive);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    //TODO  2020-01-05
    let result = read_sensitive_words();
    if let Ok(_) = result {
        println!("read OK");
    }

    println!("length of set is {}",SENSITIVE_WORDS_SET.len());

    Ok(())
}
///
///  在指定的字符串contents中查找字符串query，并返回Vec
///
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

///
///  在指定的字符串contents中查找字符串query（大小写不敏感），并返回Vec
///
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_query() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

use std::collections::HashSet;
fn read_sensitive_words() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("sensitive_words.txt")?;
    let sensitive_words = contents.lines();

    let mut sensitive_words_map = HashSet::<&str>::new();
    for line in sensitive_words {
        println!("{}", line);
        sensitive_words_map.insert(&line);
    }

    println!("{:?}", sensitive_words_map);
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    //    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    Ok(())
}
