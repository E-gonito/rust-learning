// import io library from stf library into scope for input
use std::io;

// Main is entry point of the program
fn main() {
    println!("Guess a number: ");
    // new(); is a function that creates a growable, UTF-8 encoded bit of text
    // :: (Scope Resolution Operator) means accessing the String namespace
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");
    println!("You guessed: {guess}");
}
