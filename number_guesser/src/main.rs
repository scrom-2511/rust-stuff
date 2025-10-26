use rand::Rng;
use std::io;

fn main() {
    let random_number = rand::rng().random_range(0..=100);

    loop {
        let mut input = String::new();

        println!("Please enter a number: ");
        io::stdin().read_line(&mut input).expect("Unable to read line!");

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                println!("Please enter a valid number!");
                continue;
            }
        };

        if guess == random_number {
            println!("🎉 Your guess is correct!");
            break;
        } else if guess < random_number {
            println!("Too small! Try again.");
        } else {
            println!("Too big! Try again.");
        }
    }
}
