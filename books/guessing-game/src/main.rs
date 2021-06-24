use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Quick Notes:
    // - snake_case
    // - cargo fmt » formatter

    println!("Guess the number!");

    loop {
        println!("Kindly enter your guess.");

        let secret = rand::thread_rng().gen_range(1..101);

        println!("The secret number is: {}", secret);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("I was unable to read the line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("Your guess is {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Oops! Too small!"),
            Ordering::Greater => println!("Oops! Too big!"),
            Ordering::Equal => {
                println!("Perfecto! You win boss!");
                let mut restart = String::new();
                println!("Would you like to play again? [yes / no]");
                io::stdin()
                    .read_line(&mut restart)
                    .expect("I was unable to read the line");
                let restart = restart.trim().to_lowercase();
                if restart != "yes" {
                    break;
                }
            }
        }
    }
}
