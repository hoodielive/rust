use std::io; 
mod module; 
use module::foo;
fn main() {
    foo(); 
    println!("Welcome to the Guessing game!");
    println!("Input Guess: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess); 
    
    println!("You guessed: {}", guess); 
}
