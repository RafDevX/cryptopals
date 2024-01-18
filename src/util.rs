use std::io::{stdin, BufRead};

/// Reads one line from stdin and returns it.
///
/// # Panics
///
/// Panics if there are no input lines or if it fails to read a line.
pub fn read_one_line() -> String {
    stdin()
        .lock()
        .lines()
        .next()
        .expect("No lines in input")
        .expect("Failed to read line")
}
