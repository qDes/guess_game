use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please, enter your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Can't read string");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is too small!"),
            Ordering::Greater => println!("The number is too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }


}
