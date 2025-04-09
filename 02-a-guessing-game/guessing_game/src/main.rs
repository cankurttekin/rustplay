use std::io;
use rand::Rng; // rand crate for random number generatio

fn main() {
    println!("Guess the number game(is it really a game?)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please enter a number:");
    // in rust vars are immutable by default
    // to make a var mutable we used the mut keyword
    let mut guess = String::new(); // empty string bound to mutable var guess

    io::stdin()
        .read_line(&mut guess) // & indicates a reference
                               // lets you access the value without copy to mem multiple times
                               // referances are immutable by default to make mutable &mut guess
                               // rather than &guess
        .expect("Failed to read line"); // if read_line return Err, panic with message

    println!("You guessed: {}", guess);
}
