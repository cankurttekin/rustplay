use std::io;
use std::cmp::Ordering; // for comparison
use rand::Rng; // rand crate for random number generation

fn main() {
    println!("Guess the number game (is it really a game?)");

    let secret_number = rand::thread_rng().gen_range(1..=100); // rust defaults to i32 

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please enter a guess:");

        // in rust vars are immutable by default
        // to make a var mutable we used the mut keyword
        let mut guess = String::new(); // empty string bound to mutable var guess

        io::stdin()
            .read_line(&mut guess) // & indicates a reference
                                   // lets you access the value without copy to mem multiple times
                                   // references are immutable by default to make mutable &mut guess
                                   // rather than &guess
            .expect("Failed to read line"); // if read_line returns Err, panic with message

        // parse converts string to u32, returns Result; trim removes whitespace
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if parse() returns Ok, bind num to guess
            Err(_) => {
                println!("Not a number! Please enter a number.");
                continue; // continue to next iteration of loop
            }
        };

        println!("You guessed: {}", guess);

        // breaks on equality
        // otherwise prints too small or too big
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
