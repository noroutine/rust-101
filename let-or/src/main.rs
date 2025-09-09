#![allow(dead_code)]

#[derive(Debug)]
enum MyEnum {
    A(i32),
    B(i32),
    // C
}

fn main() {
    let value_a = MyEnum::A(-42);
    let value_b = MyEnum::B(42);
    let (MyEnum::A(ref x) | MyEnum::B(ref x)) = value_a; // Works
    println!("A: {:?} B: {:?} x: {:?}", value_a, value_b, x);

    let x = 5;
    let foo = 5;
    let y = false;

    match x {
        x2 @ 4 => println!("yes, x2 {x2}!"),
        x3 @ 5 => println!("yes, foo {x3}"), 
        6 if y => println!("yes"),
        _ => println!("no"),
    }
}
