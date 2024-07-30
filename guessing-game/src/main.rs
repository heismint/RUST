use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Processing a guess
    println!("Guess the number!");

    // Generating a number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number); //This line prints the secret number, it kills the fun

    // Allowing multiple guesses with looping
    loop {
        println!("Please input your guess.");
        // Storing values with variables
        let mut guess = String::new(); // immutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the string the program reads as input into a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Handle invalid input
        };

        // Printing values with println! placeholder
        println!("You guessed: {}", guess);

        // Comparing the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!"); // Quitting after a correct guess
                break;
            }
        }
    }
}
