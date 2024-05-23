// use is importing from the standard library the io package which is used to obtain user input and print the result as output
use std::io;
// use is importing from the standard library a type called Ordering which is another enum with variants Less, Greater, Equal, cmp = compare
use std::cmp::Ordering;

// use is importing from the rand dependency installed in Cargo.toml, the Rng trait defines methods that random number generators implement, this trait is being scoped here in order to use the methods
use rand::Rng;

//fn syntax is declaring a new function, the empty parenthenses indicate that there are no parameters, the curly brace starts the body
fn main() {
    // println! is a macro which will print a string to screen
    println!("Guess the number!");
    // rand::thread_rng is a function giving us a particular rng local to the current thread of execution and seeded by the operating system, the gen_range method is called on the generator taking in a range expression the syntax being start..=end and is inclusive, the command cargo doc --open will build documentation for all dependencies installed and open it with your browser
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
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

        // creating another variable named guess, rather, shadowing reusing the guess variable name rather than having to create two unique variables, the guess within the expression refers to the original value, trim eleminates whitespace, this needs to happen in order to compare the string to the u32 (unsigned 32-bit number), the parse method converts a string to another type, the guess: u32 is annotating the variables type, parse returns a Result type because if the string contained characters that couldnt be converted to a number it can fail; the _ in the Err is a catchall value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the curly brackets around guess are a placeholder, if printing a variable name the variable should go inside the brackets, if printing a result of an expression place empty curlies in the format string, follow the format string with a comma-separated list of expressions to print in each empty curly backet in the same order ex: (y + 2 = {}, y + 2)

        println!("You guess: {guess}");

        // the cmp method compares two values and can be called on anything that can be compared, it takes a reference to whatever you want to compare with (&secret_number)
        // a match expression is made up of arms, an arm is a pattern to match against and the code that should be run if the value given to match fits the arm's pattern, rust takes the value given to match and sequentially goes through each arm's pattern, once a pattern matches the match expression ends

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
