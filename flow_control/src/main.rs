// std is standard rust libs so no importing dependencies
use std::io::{stdin, stdout, Write};

// this is a dependency that is added to cargo and handles
// randonm number generation
use rand::Rng;

fn main() {
    guessing_game();
}

fn guessing_game() {
    let mut rng = rand::thread_rng();
    let mut attempts = 3;
    let number = rng.gen_range(0..10);
    let mut guess = get_guess();

    while attempts >= 1 {
        if !check_guess(number, guess) {
            attempts -= 1;
            println!("Attempts left: {}", attempts);
            if attempts > 0 {
                guess = get_guess();
            } else {
                game_state("l");
            }
        } else {
            game_state("w");
            return;
        }
    }
    return;
}

fn check_guess(a: i32, g: i32) -> bool {
    // println!("Answer: {} | Guess: {}", a, g);
    if g < a {
        println!("Higher than {}", g);
    } else if g > a {
        println!("Lower than {}", g);
    } else {
        return true;
    }
    return false;
}

fn get_guess() -> i32 {
    let mut input = String::new();
    print!("\nGuess a number from 1 - 10: ");
    stdout().flush();
    stdin().read_line(&mut input);
    let guess: i32 = input.trim().parse().unwrap();
    return guess;
}

fn game_state(status: &str) {
    if status == "w" {
        println!("You win!");
    } else if status == "l" {
        println!("You lose!");
    }
}
