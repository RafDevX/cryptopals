use crate::util;

use super::chall02;

pub fn solve_chall03() {
    let lines = util::read_hex_lines_stdin();
    let input = lines.first().expect("No first operand provided");
    let (_, key, msg) = find_single_byte_key(input).expect("No single-byte key found");
    println!("Key: {key:#02x}");
    println!("{}", util::bytes_to_hex(&msg))
}

pub fn find_single_byte_key(c: &[u8]) -> Option<(u8, u8, Vec<u8>)> {
    let mut top_candidate = None;

    for k in u8::MIN..=u8::MAX {
        let mut augmented_key = vec![];
        augmented_key.resize(c.len(), k);

        let candidate = chall02::xor(c, &augmented_key);
        let score = score_english_phrase(&candidate);

        match top_candidate {
            Some((top_score, _, _)) if top_score >= score => {} // ignore
            _ => top_candidate = Some((score, k, candidate)),
        }
    }

    top_candidate
}

fn score_english_phrase(phrase: &[u8]) -> u8 {
    phrase.iter().fold(0, |acc, x| {
        if x.is_ascii_alphabetic() || *x == b' ' {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let (_, key, msg) = find_single_byte_key(&util::hex_to_bytes(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        ))
        .expect("No single-byte key found");
        assert_eq!("58", util::bytes_to_hex(&[key]));
        assert_eq!("Cooking MC's like a pound of bacon".as_bytes(), msg);
    }
}
