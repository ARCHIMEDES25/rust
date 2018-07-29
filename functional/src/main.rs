use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    //let x = vec![1, 2, 3];
    // https://doc.rust-lang.org/book/second-edition/ch13-01-closures.html#capturing-the-environment-with-closures
    // If you want to force the closure to take ownership of the values 
    // it uses in the environment, you can use the move keyword 
    // before the parameter list. 
    // This technique is mostly useful when passing a closure to a new thread 
    // to move the data so it’s owned by the new thread.

    //let equal_to_x = move |z| z == x;

    //println!("can't use x here: {:?}", x);

    //let y = vec![1, 2, 3];

    //assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {

    //this syntax was chosen because of its similarity to closure 
    // definitions in Smalltalk and Ruby
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[derive(Debug)]
struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

// https://doc.rust-lang.org/book/second-edition/ch13-01-closures.html#limitations-of-the-cacher-implementation
impl<T> Cacher<T> 
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}