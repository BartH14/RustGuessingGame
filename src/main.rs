use std::cmp::Ordering;
use std::io::stdin;
use rand;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number was: {secret_number}");
                break;
            }
        }
    }
}
