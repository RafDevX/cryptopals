use std::{
    fmt::Write,
    io::{stdin, BufRead},
};

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

/// Reads hex lines from stdin and returns them as byte sequences.
///
/// # Panics
///
/// Panics if it fails to read a line, or if any line is an invalid hex sequence.
pub fn read_hex_lines() -> Vec<Vec<u8>> {
    stdin()
        .lock()
        .lines()
        .map(|x| x.expect("Failed to read line"))
        .map(|x| hex_to_bytes(&x))
        .collect()
}

/// Converts a hex sequence into a vector of bytes.
///
/// # Panics
///
/// Panics if an invalid hex sequence is provided.
pub fn hex_to_bytes(line: &str) -> Vec<u8> {
    if line.len() % 2 != 0 {
        panic!("Invalid hex sequence (odd)")
    }

    let mut result = vec![];

    for i in (0..line.len()).step_by(2) {
        let s = &line[i..(i + 2)];
        let byte = u8::from_str_radix(s, 16).expect("Invalid hex value");
        result.push(byte);
    }

    result
}

/// Converts a vector of bytes into a hex sequence.
pub fn bytes_to_hex(seq: &[u8]) -> String {
    seq.iter().fold(String::new(), |mut out, val| {
        write!(out, "{val:02x}").unwrap();
        out
    })
}
