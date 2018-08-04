use std::ops::Deref;
use List:: {Cons, Nil};

use std::rc::Rc;

fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    let x = 5;
    //y equal to a reference to x
    let y = &x;
    // or
    //let y = Box::new(x);
    let y = MyBox::new(x);

    println!("x is {}", x);
    println!("y is {}", *y);

    assert_eq!(5, x);
    // *y to follow the reference to the value it’s pointing to (hence dereference). 
    // Once we dereference y, we have access to the integer value y is pointing to that 
    // we can compare with 5
    assert_eq!(5, *y);

    let c = CustomSmartPointer {  data: String::from("HELLO")};
    let d = CustomSmartPointer {  data: String::from("THERE")};
    let e = CustomSmartPointer {  data: String::from("AGAIN")};    

    println!("CustomSmartPointers created...");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}


#[derive(Debug)]
// enum List {
//     Cons(i32, List),
//     Nil,
// }

//https://doc.rust-lang.org/book/second-edition/ch15-01-box.html#using-boxt-to-point-to-data-on-the-heap
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

//https://doc.rust-lang.org/book/second-edition/ch15-02-deref.html#how-deref-coercion-interacts-with-mutability


#[derive(Debug)]
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

use List2::{Cons as Conse, Nil as Nile};


fn rc_trait() {
    // This doesn't compile
    // let a = Cons(5,
    //     Box::new(Cons(10,
    //         Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(Conse(5, Rc::new(Conse(10, Rc::new(Nile)))));
    let b = Conse(3, Rc::clone(&a));
    let c = Conse(4, Rc::clone(&a));
}