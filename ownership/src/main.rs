fn main() {
    
    {                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
    } 

    //ownership_with_string();
    //ownership_with_string_move();
    //ownership_with_string_clone();
    //ownership_with_functions();
    //ownership_with_return_and_scope();
    //references_and_borrowing();
    //mutable_references();
    slices();
}

fn ownership_with_string(){
    //The double colon (::) is an operator that allows us to namespace this particular from function 
    //under the String type
    //let s = String::from("hello");

    let mut s = String::from("hello"); // s is valid from this point forward
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
} // this scope is now over, and s is no
// longer valid


// fn ownership_with_string_move(){
// let s1 = String::from("hello");
// let s2 = s1;

// println!("{}, world!", s1); //this will error because Rust prevents you from using the invalidated reference:
// }

fn ownership_with_string_clone(){
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
} 

fn ownership_with_functions(){

     let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward


    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    }
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

  fn ownership_with_return_and_scope(){

      let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

        fn gives_ownership() -> String {             // gives_ownership will move its
                                                // return value into the function
                                                // that calls it

        let some_string = String::from("hello"); // some_string comes into scope

        some_string                              // some_string is returned and
                                                // moves out to the calling
                                                // function
    }

    // takes_and_gives_back will take a String and return one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                        // scope

        a_string  // a_string is returned and moves out to the calling function
    }
  }// Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

  fn references_and_borrowing(){
      let s1 = String::from("hello");

      //change(&s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {// s is a reference to a String
    s.len()
    }
    // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

    //Attempting to modify a borrowed value, will error
    // fn change(some_string: &String) {
    //     some_string.push_str(", world");
    // }
  }

  fn mutable_references(){
      let mut s = String::from("hello");
      //cannot borrow `s` as mutable more than once at a time
    //   let r1 = &mut s;
    //   let r2 = &mut s;
    
    //
    //let r1 = &s; // no problem
    //let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM

    {
    let r1 = &mut s;

    } // r1 goes out of scope here, so we can make a new reference with no problems.

    //let r2 = &mut s;

    change(&mut s);

    fn change(some_string: &mut String) {
    some_string.push_str(", world");
    }
  }

  fn dangling_reference(){
      let reference_to_nothing = no_dangle();

//     fn dangle() -> &String {
//         let s = String::from("hello");
//         &s// we return a reference to the String, s
//     }// // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }
  }

  fn slices(){
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let len = s.len();
    //same
    let slice = &s[3..len];
    let slice = &s[3..];

    //same
    let slice = &s[0..len];
    let slice = &s[..];

    let word = first_word(&s[..]);
    //or 
    //let word = first_word(&s);
    // errors mutable borrow occurs here
    //s.clear();
    first_word("jgjg kgfkgk");

    println!("first word is : {}", word);


    // fn first_word(s: &String) -> &str {, improved signature below...
    //string literals are slices..
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
  }