### README.md

# File Analyzer

File Analyzer is a simple Rust command-line application that reads a text file and provides various statistics such as the number of lines, characters, words, unique words, and the most frequent words.

## Features

- Count the number of lines in a file.
- Count the number of characters in a file.
- Count the number of words in a file.
- Count the number of unique words in a file.
- Display the most frequent words in a file.

## Requirements

- Rust programming language (Ensure you have Rust installed. You can install it from [here](https://www.rust-lang.org/)).

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/your-username/file-analyzer.git
   ```
2. Navigate to the project directory:
   ```sh
   cd file-analyzer
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

To use the File Analyzer, run the executable and provide the path to the text file you want to analyze as an argument.

```sh
cargo run --release <file>
```

Example:

```sh
cargo run --release sample.txt
```

### Output

The output will display the following information:

- Number of lines in the file.
- Number of characters in the file.
- Number of words in the file.
- Number of unique words in the file.
- The 5 most frequent words in the file and their frequencies.

### Example Output

```
Number of lines: 42
Number of characters: 1234
Number of words: 245
Number of unique words: 123

Most frequent words:
the: 15
and: 12
to: 10
of: 8
a: 7
```

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
```

### Usage Guide

Here is a step-by-step usage guide for running your Rust application:

1. **Ensure Rust is Installed**: Make sure you have Rust installed on your system. If not, install it from the official Rust website.

2. **Clone the Repository**: Clone your repository to your local machine.
   ```sh
   git clone https://github.com/your-username/file-analyzer.git
   cd file-analyzer
   ```

3. **Build the Project**: Use Cargo to build the project in release mode.
   ```sh
   cargo build --release
   ```

4. **Run the Application**: Execute the compiled binary and provide the path to the text file you want to analyze.
   ```sh
   cargo run --release <file>
   ```
   Replace `<file>` with the path to your text file.

5. **View the Output**: The application will output various statistics about the file, including the number of lines, characters, words, unique words, and the most frequent words.

### Example

If you have a file named `sample.txt`, you would run:
```sh
cargo run --release sample.txt
```

And you would see an output similar to:
```
Number of lines: 42
Number of characters: 1234
Number of words: 245
Number of unique words: 123

Most frequent words:
the: 15
and: 12
to: 10
of: 8
a: 7
```