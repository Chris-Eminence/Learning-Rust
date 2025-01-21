use rand::Rng; // Import the `Rng` trait to generate random numbers.
use std::cmp::Ordering; // Import the `Ordering` enum for comparison results.
use std::io; // Import `io` for standard input/output.

fn main() {
    // Print a welcome message for the game.
    println!("This is a Guessing Game");

    loop {
        // Generate a random secret number between 1 and 100 (inclusive).
        let secrete_number = rand::thread_rng().gen_range(1..=100);
        println!("Guess a Number: ");

        // Create a mutable string to store the user's guess.
        let mut guess = String::new();

        // Read user input and handle errors if the input can't be read.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read the Input/Line");

        // Parse the user's input string into an unsigned 32-bit integer.
        // If parsing fails, skip the current iteration of the loop.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Successfully parsed; use the number.
            Err(_) => continue, // Parsing failed; continue to the next iteration.
        };

        // Print the secret number (for debugging or demonstration purposes).
        println!("The secret number is: {secrete_number}");

        // Print the user's guess.
        println!("You guessed: {}", guess);

        // Compare the user's guess with the secret number.
        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("Too small!"), // Guess is smaller than the secret number.
            Ordering::Greater => println!("Too big!"), // Guess is larger than the secret number.
            Ordering::Equal => {
                println!("You win!"); // Guess is correct; user wins the game.
                break; // Exit the loop and end the game.
            }
        }
    }
}
