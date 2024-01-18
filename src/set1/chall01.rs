use std::cmp::{max, min};

use crate::util;

pub fn solve_chall01() {
    let input = util::read_one_line();
    let output = hex_to_base64(input);
    println!("{output}")
}

enum Base64Char {
    Char(u8),
    Padding,
}

impl From<&Base64Char> for char {
    fn from(value: &Base64Char) -> Self {
        match value {
            Base64Char::Padding => '=',
            Base64Char::Char(n) => match n {
                0..=25 => char::from(b'A' + n),
                26..=51 => char::from(b'a' + (n - 26)),
                52..=61 => char::from(b'0' + (n - 52)),
                62 => '+',
                63 => '/',
                _ => unimplemented!("Invalid base64 character"),
            },
        }
    }
}

fn hex_to_base64(input: String) -> String {
    let mut result = vec![];

    // manually iterating instead of using .chunks() so we have s: &str
    for i in (0..input.len()).step_by(6) {
        let j = min(i + 6, input.len());
        let s = &input[i..j];

        if s.len() % 2 != 0 {
            panic!("Invalid hex sequence (odd)")
        }
        let n_bytes = s.len() / 2;
        let padding = max(0, 3 - n_bytes); // 2 for n_bytes=1, 1 for n_bytes=2, 0 otherwise
        let num = u32::from_str_radix(s, 16).expect("Invalid hex") << (2 * padding);

        let mut mask = 0b111111 << (18 - (8 * padding)); // 18, 10, 2 [padding=0,1,2]

        while mask > 0 {
            let sextet = ((num & mask) >> mask.trailing_zeros()) as u8;
            result.push(Base64Char::Char(sextet));
            mask >>= 6;
        }

        for _ in 0..padding {
            result.push(Base64Char::Padding);
        }
    }

    result.iter().map(char::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_owned())
        )
    }
}
