use std::io::{self, Write};

fn main() {
    // Print a message asking for input
    print!("Please enter a string: ");
    io::stdout().flush().unwrap();

    // Read the input string from the user
    let mut input = String::new();  // Mutable string
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any leading or trailing whitespace
    let input: &str = input.trim();  // String slice

    // Split the input string by whitespace and count the number of words
    let words: Vec<&str> = input.split_whitespace().collect();  // Vector of string slices
    let word_count: usize = words.len();  // Unsigned integer

    // Print the word count
    println!("The input contains {} words:", word_count);

    // Print each word on a new line
    for word in words {
        println!("{}", word);
    }
}
