use std::fmt;
use std::fmt::Display;

/// Person struct
#[derive(Debug)]
struct Person {
    age: i32,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.age)
    }
}

fn main() {
    println!("Hello World");
    println!("I'm a Crustacean");

    let person = Person { age: 18 };
    println!("{}", person);
}