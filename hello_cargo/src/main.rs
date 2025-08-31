#![allow(dead_code)]
#![allow(unused_imports)]

mod shapes;

use std::{error::Error, fmt, path::PathBuf};

use shapes::Rectangle;
// use crate::shapes::Rectangle;

#[derive(Debug)]
struct ReadConfigError {
    path: PathBuf,
}

impl fmt::Display for ReadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.path.display();
        write!(f, "unable to read configuration at {path}")
    }
}

impl Error for ReadConfigError {}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let mut huge_array = [1; 2_000_000]; // Still stack!
    huge_array[0] = 3;
    println!("Array created! First element: {}", huge_array[0]);
    println!("Last element: {}", huge_array[huge_array.len() - 1]);

    let mut huge_vector = vec![1; 10_000_000]; // heap
    huge_vector[0] = 3;
    println!("Vector created! First element: {}", huge_vector[0]);
    println!("Last element: {}", huge_vector[huge_vector.len() - 1]);

    let small_square = Rectangle::square(5);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "Can rect1 hold small square? {}",
        rect1.can_hold(&small_square)
    );


    // let greeting_file = std::fs::File::open("hello.txt")?;

    // Ok(())
    Err(Box::new(ReadConfigError {
        path: PathBuf::from("config.toml"),
    }))
}
