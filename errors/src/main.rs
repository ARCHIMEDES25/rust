use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    //panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];
    //verbose();

    read_username_from_file_simplified();
    
}

fn verbose() {
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     },
    // };

    //or
    let f = match f {
        Ok(file) => file,
        // match guard: it’s an extra condition on a match arm that further refines the arm’s pattern
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // The ref in the pattern is needed so error is not moved into the guard condition 
            // but is merely referenced by it, in the context of a pattern, 
            // & matches a reference and gives you its value, but ref matches a value and gives you a reference to it
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Problem creating file: {:?}", e);
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        },
    };
}

fn shortcut() {
    let f = File::open("hello.txt").unwrap();

    //or
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_simplified() -> Result<String, io::Error> {
    // further detauls of ?,
    //https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    //or   
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    //The ? Operator Can Only Be Used in Functions That Return Result
}


pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value:cu32) -> Guess {
        if value< 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
