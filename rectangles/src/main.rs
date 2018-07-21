fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples((width1, height1))
    );

    let rect = Rectangle { width: 30, height: 50 };

    let rect2 = Rectangle { width: 5, height: 5 };
    let rect3 = Rectangle { width: 60, height: 55 };

    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);

    println!("Area is {}", rect.area());

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    //associated function usage
    let square = Rectangle::square(5);

    println!("Square Areas is {:#?}", square);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_struct(&rect)
    // );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//attibute added top opt in pitn out debuggin information
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
    //associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

//immutable borrow of struct Rectangle
//main reactins owenership of Rectangle instance and it's dropped once it's out of scope...
fn area_struct(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}