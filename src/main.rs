use std::io::{self, Write};
use rand::Rng;

fn main() {
    let number = random_number();
    let mut attempt = 0;

    loop {
        let (guess, err_msg) = user_input();
        if !err_msg.is_empty() {
            println!("{}\n", err_msg);
            continue
        }

        attempt += 1;

        if check_number(&number, &guess) {
            println!("\n--- Congratulation ---");
            println!("Result Number : {}", number);
            println!("Your Guess : {}", guess);
            println!("Attempt : {}", attempt);
            break
        }
        if attempt >= 5 {
            println!("\n--- Oh No! You Failed ---");
            println!("Result Number : {}", number);
            println!("Your Guess : {}", guess);
            println!("Attempt : {}", attempt);
            break
        }

        println!();
    }
}

fn check_number(number: &i32, guess: &i32) -> bool {
    if guess < number {
        println!("Your number less than the result");
        false
    } else if guess > number {
        println!("Your number greater than result");
        false
    } else {
        true
    }
}

fn random_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

fn user_input() -> (i32, String) {
    print!("Enter your guess (1-100) : ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let trim_input = input.trim();

    let number: i32 = match trim_input.parse() {
        Ok(num) => num,
        Err(_) => {
            return (0, String::from("Please insert only a number"))
        }
    };

    if number < 1 || number > 100 {
        return (0, String::from("Invalid number (1-100)"))
    }

    (number, String::from(""))
}