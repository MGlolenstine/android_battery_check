use std::io::{stdin, stdout, Write};

/// Prompts user for index and keeps asking until a parsable number is entered.
/// # Panics
///
/// Will panic if reading from [stdin] fails.
#[must_use]
pub fn get_index() -> usize {
    println!();
    let mut input = ask_user();
    while input.parse::<usize>().is_err() {
        input = ask_user();
        stdin().read_line(&mut input).unwrap();
    }
    input.parse::<usize>().unwrap()
}

fn ask_user() -> String {
    let mut input = String::new();
    print();
    stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}

fn print() {
    print!("> ");
    stdout().flush().expect("Failed to flush stdout?");
}
