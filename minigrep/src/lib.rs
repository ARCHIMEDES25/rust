use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get any query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // let mut f = File::open(config.filename).expect("file not found");
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    // f.read_to_string(&mut contents)
    // .expect("There was a problem reading file");
    f.read_to_string(&mut contents)?;  

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }  else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// In this case, we indicate that the returned vector 
// should contain string slices that reference slices of the argument contents 
// (rather than the argument query)

/// Searchs query in the given contents collection.
///
/// # Examples
///
/// ```
///  let query = "duct";
///         let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";

///        assert_eq!(
///             vec!["safe, fast, productive."],
///            search(query, contents)
///        );
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();
    //let query = query.to_lowercase();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me!!!";

        assert_eq!(
            vec!["Rust:", "Trust me!!!"],
            search_case_insensitive(query, contents)
        );
    }

//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(
//             vec!["safe, fast, productive."],
//             search(query, contents)
//         );
//     }
}