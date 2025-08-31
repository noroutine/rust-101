use rand::Rng;
use std::cmp::Ordering::*;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    // let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let parse_result = guess.trim().parse::<u32>();
        let guess = match parse_result {
            Ok(num) => num,
            Err(_) => {
                println!("Enter the number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
