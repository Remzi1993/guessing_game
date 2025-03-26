use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::Rng;
use console::*;

enum ColorCode {
    Error = 9,
    Success = 10,
    Info = 14,
}

impl ColorCode {
    fn value(&self) -> u8 {
        match *self {
            ColorCode::Error => 9,
            ColorCode::Success => 10,
            ColorCode::Info => 14,
        }
    }
}

// Define constants for the guessing range
const MIN_GUESS: u32 = 1;
const MAX_GUESS: u32 = 100;

fn main() {
    println!("Guess the number! - random number between {} and {}",
             style(MIN_GUESS).color256(ColorCode::Info.value()),
             style(MAX_GUESS).color256(ColorCode::Info.value()));

    let secret_number = rand::rng().random_range(MIN_GUESS..MAX_GUESS + 1);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if !(MIN_GUESS..=MAX_GUESS).contains(&num) {
                    println!("{}", style(format!("Please enter a number between {} and {}!", MIN_GUESS, MAX_GUESS))
                        .color256(ColorCode::Error.value()));
                    continue;
                }
                num
            },
            Err(_) => {
                println!("{}", style("Invalid input. Please enter a valid number!").color256(ColorCode::Error.value()));
                continue;
            }
        };

        println!("{} {}", style("\nYou guessed:").color256(ColorCode::Info.value()), guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", style("Too small!").color256(ColorCode::Error.value())),
            Ordering::Greater => println!("{}", style("Too big!").color256(ColorCode::Error.value())),
            Ordering::Equal => {
                println!("{}", style("You win!").color256(ColorCode::Success.value()));
                break;
            }
        }

        println!();
    }

    println!("\nPress {} to close the program.", style("Enter").color256(ColorCode::Info.value()));
    let mut exit_input = String::new();
    io::stdin().read_line(&mut exit_input).expect("Failed to read line");
}