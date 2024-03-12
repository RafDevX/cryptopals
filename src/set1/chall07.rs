use std::io;

use openssl::symm;

use crate::util;

use super::chall06;

pub fn solve_chall07() {
    let input = io::read_to_string(io::stdin()).expect("Failed to read input");
    let ciphertext = chall06::base64_decode(&input).expect("Failed to base64-decode");
    let key = "YELLOW SUBMARINE".as_bytes();
    let plaintext = decrypt_aes_ecb(&ciphertext, key).expect("Failed to decrypt");

    println!("Plaintext: {}", util::bytes_to_hex(&plaintext));
    println!(
        "ASCII: {}",
        plaintext
            .iter()
            .cloned()
            .map(char::from)
            .collect::<String>()
    );
}

pub fn decrypt_aes_ecb(
    ciphertext: &[u8],
    key: &[u8],
) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = symm::Cipher::aes_128_ecb();
    let mut decrypter = symm::Crypter::new(cipher, symm::Mode::Decrypt, key, None)?;
    decrypter.pad(false);

    let mut plaintext = vec![0; 2896]; // openssl complains if <2896
    decrypter.update(ciphertext, &mut plaintext)?;

    plaintext.truncate(ciphertext.len());
    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let reader = util::get_file_reader("data/7.txt");
        let input = io::read_to_string(reader).expect("Failed to read file");
        let ciphertext = chall06::base64_decode(&input).expect("Failed to base64-decode");
        let key = "YELLOW SUBMARINE".as_bytes();
        let plaintext = decrypt_aes_ecb(&ciphertext, key).expect("Failed to decrypt");

        // message is too long to check the entire thing here; this is a heuristic
        let msg_needle = "Play that funky music".as_bytes();
        assert_eq!(6, util::count_occurrences(&plaintext, msg_needle));
    }
}
