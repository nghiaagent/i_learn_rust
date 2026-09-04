use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    let mut guess = String::new(); // declare `guess` as a mutable variable to accept inputs. Bind it to a new String. new() is a function associated to the String type.

    io::stdin()
        .read_line(&mut guess) // take stdin and append that into the a mutable reference of guess
        .expect("Failed to read line"); // the .read_line method returns a Result enum containing error info. If it returns an Err, stop the program. Basically a try-catch block.

    println!("You guessed: {guess}"); // the {} prints the value of guess.
}