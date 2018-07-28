extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;


// use std::env;
// use std::fs::File;
// use std::io::prelude::*;
// use std::process;
// use std::error::Error;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let filename = &args[2];
    // // println!("{:?}", args);
    // println!("{}", query);
    // println!("{}", filename);

    // let mut f = File::open(filename).expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    // .expect("There was a problem reading file");

    // println!("With text:\n{}", contents);


    let args: Vec<String> = env::args().collect();
    //let (query, filename) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args {}", err);
        process::exit(1);
    });
    
    // println!("{}", config.query);
    // println!("{}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Boom!!!: {}", e);
        process::exit(1);
    }
}

// #[derive(Debug)]
// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {

//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }

// fn run(config: Config) -> Result<(), Box<Error>> {
//     // let mut f = File::open(config.filename).expect("file not found");
//     let mut f = File::open(config.filename)?;

//     let mut contents = String::new();
//     // f.read_to_string(&mut contents)
//     // .expect("There was a problem reading file");
//     f.read_to_string(&mut contents)?;    

//     println!("With text:\n{}", contents);
//     Ok(())
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1];
//     let filename = &args[2];

//     (query, filename)
// }