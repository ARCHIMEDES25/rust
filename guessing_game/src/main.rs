extern crate rand;

use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    another_function(5);

    let x = plus_one(5);

    println!("The value of x is {}", x);

    branches();

    funky_branch();

    loop_is_fun();
    // println!("Guess the number!");
    // const MAX_POINTS: u32 = 100_000;
    // let heart_eyed_cat = '😻';

    // let secret_number = thread_rng().gen_range(1, 101);

    // let tuple: (u32, char, u32) = (MAX_POINTS, heart_eyed_cat, secret_number);

    // let (_x, y, _z) = tuple;

    // println!("The secret number is: {} {} {}", tuple.2, tuple.0, y);

    // loop {
    //     println!("Please input your guess.");
    //     //mutable variable
    //     let mut guess = String::new();//static method

    //     io::stdin().read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };         

    //     println!("You guessed: {}", guess);

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }    
}

fn another_function(x: i32) {
    // expression, no ending semicolon
    let y = {
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    // no ending semicolon
    x + 1
}

fn branches() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn funky_branch(){

    let condition = true;

    let number = if condition {
        5
    } else {
        6
        //"six", this isn't valid
    };

    println!("The value of number is: {}", number);
}


fn loop_is_fun(){
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //Range This is nicer....
    for number in (1..8).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}