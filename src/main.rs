use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number between 1 and 1000 (inclusive)");

    let var_name = 1000;
    let secret_number: u32 = rand::thread_rng().gen_range(1..=var_name);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter an integer");
                continue;
            }
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

    }
}
