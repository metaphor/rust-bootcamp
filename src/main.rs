use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut guess_count: u32 = 0;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess_input = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        guess_count = guess_count + 1;
        println!("You guessed: {}", guess_input);

        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number! {}", guess_input);
                continue;
            }
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    println!("Your guess count {}", guess_count);
}
