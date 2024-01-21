use std::io;

use itertools::Itertools;

use crate::util;

use super::{chall01, chall03, chall05};

pub fn solve_chall06() {
    let input = io::read_to_string(io::stdin()).expect("Failed to read input");
    let ciphertext = base64_decode(&input).expect("Failed to base64-decode");
    let (key, msg) = decrypt_repeating_key_xor(&ciphertext).expect("Failed to decrypt");

    println!(
        "Key: {} [ len = {}; ASCII: {} ]",
        util::bytes_to_hex(&key),
        key.len(),
        key.iter().cloned().map(char::from).collect::<String>()
    );
    println!("Message: {}", util::bytes_to_hex(&msg));
    println!(
        "ASCII: {}",
        msg.iter().cloned().map(char::from).collect::<String>()
    );
}

impl From<&char> for chall01::Base64Char {
    fn from(value: &char) -> Self {
        match value {
            'A'..='Z' => chall01::Base64Char::Char((*value as u8) - b'A'),
            'a'..='z' => chall01::Base64Char::Char((*value as u8) - b'a' + 26),
            '0'..='9' => chall01::Base64Char::Char((*value as u8) - b'0' + 52),
            '+' => chall01::Base64Char::Char(62),
            '/' => chall01::Base64Char::Char(63),
            '=' => chall01::Base64Char::Padding,
            _ => unreachable!(),
        }
    }
}

pub fn base64_decode(input: &str) -> Result<Vec<u8>, ()> {
    input
        .as_bytes()
        .iter()
        .filter(|b| b.is_ascii_alphanumeric() || **b == b'+' || **b == b'/' || **b == b'=')
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let mut count = 0;
            let mut val: u32 = 0;
            let mut padding = 0;

            for octet in chunk {
                count += 1;
                match (&char::from(*octet)).into() {
                    chall01::Base64Char::Char(n) => {
                        if padding > 0 {
                            // no non-padding allowed after padding
                            return Err(());
                        }

                        val = (val << 6) | (n as u32)
                    }
                    chall01::Base64Char::Padding => {
                        padding += 1;
                        val >>= 2
                    }
                }
            }

            if count != 4 || padding > 2 {
                Err(())
            } else {
                let mut plain = vec![];
                for i in (0..(3 - padding)).rev() {
                    plain.push((val >> (i * 8)) as u8)
                }

                Ok(plain)
            }
        })
        .flatten_ok()
        .collect()
}

fn decrypt_repeating_key_xor(ciphertext: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let key_size: usize = find_key_size(ciphertext)?.into();

    let transposed = (0..key_size).map(|i| {
        ciphertext
            .iter()
            .skip(i)
            .step_by(key_size)
            .copied()
            .collect_vec()
    });

    let key: Vec<u8> = transposed
        .map(|t_block| chall03::find_single_byte_key(&t_block).map(|(_, k, _)| k))
        .collect::<Option<_>>()?;

    let msg = chall05::repeating_key_xor(ciphertext, &key);

    Some((key, msg))
}

fn find_key_size(ciphertext: &[u8]) -> Option<u8> {
    let mut top_candidate = None;
    let mut top_score = f64::MAX;

    for candidate_size in 2..=40_u8 {
        let chunks = ciphertext.iter().chunks(candidate_size.into());
        let it = chunks
            .into_iter()
            .map(|block| block.copied().collect_vec())
            .tuple_windows();

        let mut distances_normalized = vec![];

        for (a, b) in it {
            if b.len() < a.len() {
                // ignore last chunk if it's smaller (total length not divisible by candidate_size)
                break;
            }

            distances_normalized.push(hamming_distance(&a, &b) / candidate_size);
        }

        if !distances_normalized.is_empty() {
            let sum: u64 = distances_normalized.iter().copied().map_into::<u64>().sum();
            let avg = (sum as f64) / (distances_normalized.len() as f64);
            if avg < top_score {
                top_candidate = Some(candidate_size);
                top_score = avg;
            }
        }
    }

    top_candidate
}

fn hamming_distance(a: &[u8], b: &[u8]) -> u8 {
    if a.len() != b.len() {
        panic!("Hamming distance of arguments with different length");
    }

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones() as u8)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let reader = util::get_file_reader("data/6.txt");
        let input = io::read_to_string(reader).expect("Failed to read file");
        let ciphertext = base64_decode(&input).expect("Failed to base64-decode");
        let (key, msg) =
            decrypt_repeating_key_xor(&ciphertext).expect("No single-byte encryption found");

        assert_eq!("Terminator X: Bring the noise".as_bytes(), key);
        // message is too long to check the entire thing here; this is a heuristic
        let msg_needle = "Play that funky music".as_bytes();
        assert_eq!(6, util::count_occurrences(&msg, msg_needle));
    }

    #[test]
    fn base64_decode_works() {
        assert_eq!(
            Ok("light work.".as_bytes().to_vec()),
            base64_decode("bGlnaHQgd29yay4="),
        );
        assert_eq!(
            Ok("light work".as_bytes().to_vec()),
            base64_decode("bGlnaHQgd29yaw=="),
        );
        assert_eq!(
            Ok("light wor".as_bytes().to_vec()),
            base64_decode("bGlnaHQgd29y"),
        );
    }

    #[test]
    fn base64_decode_ignores_unknown_symbols() {
        assert_eq!(
            Ok("light work".as_bytes().to_vec()),
            base64_decode("b__Glna$$$HQ€ €gd2## ##9y?aääääw=="),
        );
    }

    #[test]
    fn base64_decode_rejects_malformed() {
        assert!(base64_decode("bGl").is_err());
        assert!(base64_decode("bG=n").is_err());
        assert!(base64_decode("b===").is_err());
        assert!(base64_decode("$abc").is_err());
    }

    #[test]
    fn find_key_size_works() {
        let reader = util::get_file_reader("data/6.txt");
        let input = io::read_to_string(reader).expect("Failed to read file");
        let ciphertext = base64_decode(&input).expect("Failed to base64-decode");
        assert_eq!(Some(29), find_key_size(&ciphertext));
    }

    #[test]
    fn hamming_distance_works() {
        assert_eq!(
            37,
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes())
        )
    }
}
