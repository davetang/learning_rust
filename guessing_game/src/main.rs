/*
   https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
*/

// input/output library from the standard library
use std::io;
use rand::Rng;
// Ordering type is an enum and has the variants Less, Greater, and Equal
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // start..=end is inclusive on the lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // the loop keyword creates an infinite loop
    loop {
        println!("Please input your guess.");

        // let statement to create variable called guess
        // mut creates a mutable variable
        // String::new is a function that returns a new instance of a String
        let mut guess = String::new();

        // the stdin function handles user input
        // the read_line method works on the standard input handle
        // passing &mut guess to tell function what string to store the input
        // the & indicates that the argument is a reference
        // read_line puts stdin into a string and also returns a Result value
        // Result is an enumeration, enum, a type that can be in one of
        // multiple possible states; each possible state is a variant
        // Result's variants are Ok and Err
        // Values of the Result type, like values of any type, have methods
        // An instance of Result has an expect method
        // If this instance of Result is an Err value, expect will cause the
        // program to crash and display the message
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow the previous value of guess with a new one
        // shadowing lets us reuse the guess variable
        // trim() removes whitespace at the beginning and end
        // parse() method on strings converts a string to another type
        // the : after guess allows us to annotate the variable's type
        // u32 is an unsigned, 32-bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // the cmp method compares two values; here guess and secret_number
        // then it returns a variant of the Ordering enum
        // the match expression decides what to do based on the variant of Ordering
        // a match expression is made up of arms
        // an arm consists of a pattern to match against and the code to run
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // exit loop
                break;
            }
        }
    }
}
