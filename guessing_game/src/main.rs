use std::io; // input output module to read command line input
use std::cmp::Ordering; // utils to help us compare numbers
use rand::Rng; // create that provides a util for generating a random number

// this programs explores usage of the let, mut and match operations in Rust

fn main() {
    println!("Guess the number!");

    loop {

        let secret_number = rand::thread_rng().gen_range(1..101);

        // println!("The secret number is {}:", secret_number);

        println!("Make a guess");

        // create a var to store our string value
        let mut guess = String::new();

        io::stdin()
            // get the value from the input and set the value to the guess variable
            .read_line(&mut guess)
            // returns a Result type which will err out if not handled
            .expect("Failed to read line...");

        // overwrite the value of our original variable
        // we need to convert our string variable to a number
        // match allows us to handle any problems with the conversion ie if not a string -> number
        // parse also returns a Result type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // compare a number that we pass in (reference to it)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
    }
}
