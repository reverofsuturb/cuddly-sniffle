// use is importing from the standard library the io package which is used to obtain user input and print the result as output
use std::io;

//fn syntax is declaring a new function, the empty parenthenses indicate that there are no parameters, the curly brace starts the body

fn main() {
    // println! is a macro which will print a string to screen
    println!("Guess the number!");

    println!("Please input your guess.");
    // let used to create variable, variables are immutable by default, this is why mut is added prior, making it mutable, here guess is bound to String::new which is a function returning a new instance of a String
    let mut guess = String::new();
    /*if there was no use statement above, io::stdin could be used by calling std::io::Stdin, this is function which returns an instance of Stdin, a type that represents a handle to standard input for the terminal, .read_line calls
    the read_line method on the handle to take the user input where we are passing &mut guess as the argument indicating the string to store the input, the & indicates that the argument is a reference giving us a way to access this
    data without needing to copy the data to memory multiple times, references are also immutable by default, next we have .expect -- you can introduce whitespace by putting the . in front of a method name -- read_line returns a Result value which is considered an enumeration (enum), this is a type that can be in one of multiple possible states, each state is called a variant, the purpose of these Result types is to encode error-handling information
    Result's variants are Ok and Err, all types will have methods attached to them, expect is a method attached to Result which will return the Err if the returned instance of Result is Err */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {guess}")
}
