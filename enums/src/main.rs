fn main() {
    println!("Hello, world!");

    let four = IpAddrKind1::V4;
    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let home = IpAddrKind3::V4(127,0,0,1);

    let m = Message::Write(String::from("HI THERE"));
    m.call();

    let some_number = Some(5);
    let no_number: Option<i32> = None;

    concise_flow();
}

enum IpAddrKind1 {
    V4,
    V6,
}

enum IpAddrKind2 {
    V4(String),
    V6(String),
}

enum IpAddrKind3 {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        // do something!!!
    }
}

//Patterns that bind to state
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("lucky!!!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn placeholder() {
    //_ placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        _ => (),
    }
}

fn concise_flow() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    //an alternative
    if let Some(3) = some_u8_value {
        println!("three");
    }

    //more samples
    // let coin = Coin::Penny;
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }
}


// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind1,
//     address: String,
// }