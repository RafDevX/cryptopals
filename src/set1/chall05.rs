use std::io;

use crate::util;

pub fn solve_chall05() {
    let input = io::read_to_string(io::stdin()).expect("Failed to read input");
    let output = repeating_key_xor(input.as_bytes(), "ICE".as_bytes());
    println!("{}", util::bytes_to_hex(&output));
}

pub fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    plaintext
        .iter()
        .zip(key.iter().cycle())
        .map(|(p, k)| p ^ k)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let input = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let output = repeating_key_xor(input.as_bytes(), "ICE".as_bytes());
        assert_eq!(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
            util::bytes_to_hex(&output)
        );
    }
}
