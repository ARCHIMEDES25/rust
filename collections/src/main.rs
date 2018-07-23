use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v1 = Vec::new();
    v1.push(1);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // this will error
    // let does_not_exist = v.get(100);

    // //this will error
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];//immutable borrow here
    // v.push(6);//mtable borrow here

    iteration();

    strings();

    has_maps();
}

fn iteration() {
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![2, 4, 6];
    for i in &mut v {
        *i += 50;
    }
    // To change the value that the mutable reference refers to, 
    // we have to use the dereference operator (*) 
    // to get to the value in i before we can use the += operator 
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn multiple_types() {
    let v = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("Title")),
        SpreadsheetCell::Float(15.23),
    ];
}

fn strings() {
    let mut s = String::new();

    let s1 = "initial string".to_string();
    // or
    let s2 = String::from("initial");

    //concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    //The version of the code using format! is much easier to read 
    // and doesn’t take ownership of any of its parameters.

    println!("s1 is {}", s1);

    //indexing...
    let s1 = String::from("hello");
    //This isn't supported for more details docs 
    //https://doc.rust-lang.org/book/second-edition/ch08-02-strings.html#internal-representation
    //let h = s1[0];

    //slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }   
}

fn has_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);//Some(&10).

    for (k,v) in &scores {
        println!("{}: {}", k, v);
    }

    //The collect method gathers data into a number of collection types, 
    //including HashMap. For example, if we had the team names and initial scores in two separate vectors, 
    //we could use the zip method to create a vector of tuples where “Blue” is paired with 10, and so forth. 
    //Then we could use the collect method to turn that vector of tuples into a hash map

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    //For owned values like String, the values will be moved and 
    //the hash map will be the owner of those values
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("field_value is {}", field_value);

    //inserting value if key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    //updating values based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        //The or_insert method returns a mutable reference (&mut V) to the value for this key
        let count = map.entry(word).or_insert(0);
        // to asign to the count value we must dereference count using (*)
        *count += 1;        
    }
    println!("{:?}", map);

}
