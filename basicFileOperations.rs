use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn count_lines_and_characters(filename: &str) -> io::Result<(usize, usize)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut char_count = 0;

    for line in reader.lines() {
        let line = line?;
        line_count += 1;
        char_count += line.chars().count();
    }

    Ok((line_count, char_count))
}

fn count_words(filename: &str) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut word_count = 0;

    for line in reader.lines() {
        let line = line?;
        word_count += line.split_whitespace().count();
    }

    Ok(word_count)
}

fn count_unique_words(filename: &str) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut unique_words = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            *unique_words.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    Ok(unique_words.len())
}

fn print_most_frequent_words(filename: &str, num_words: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut word_freqs = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            *word_freqs.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    let mut sorted_freqs: Vec<_> = word_freqs.into_iter().collect();
    sorted_freqs.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    for (word, freq) in sorted_freqs.iter().take(num_words) {
        println!("{}: {}", word, freq);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }
    let filename = &args[1];

    let (line_count, char_count) = count_lines_and_characters(filename)?;
    let word_count = count_words(filename)?;
    let unique_word_count = count_unique_words(filename)?;

    println!("Number of lines: {}", line_count);
    println!("Number of characters: {}", char_count);
    println!("Number of words: {}", word_count);
    println!("Number of unique words: {}", unique_word_count);

    println!("Most frequent words:");
    print_most_frequent_words(filename, 5)?;

    Ok(())
}
