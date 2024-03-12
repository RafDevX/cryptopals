use std::io;

use itertools::Itertools;
use openssl::symm;

use crate::{
    set1::{chall02, chall06, chall07},
    util,
};

use super::chall09;

pub fn solve_chall10() {
    let input = io::read_to_string(io::stdin()).expect("Failed to read input");
    let ciphertext = chall06::base64_decode(&input).expect("Failed to base64-decode");
    let key = "YELLOW SUBMARINE".as_bytes();
    let plaintext = decrypt_aes_cbc(&ciphertext, key).expect("Failed to decrypt");

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

pub type OpenSSLResult<T> = Result<T, openssl::error::ErrorStack>;
pub fn encrypt_aes_ecb(plaintext: &[u8], key: &[u8]) -> OpenSSLResult<Vec<u8>> {
    let cipher = symm::Cipher::aes_128_ecb();
    let mut encrypter = symm::Crypter::new(cipher, symm::Mode::Encrypt, key, None)?;
    encrypter.pad(false);

    let mut ciphertext = vec![0; 96]; // openssl complains if <96
    encrypter.update(plaintext, &mut ciphertext)?;

    ciphertext.truncate(plaintext.len());
    Ok(ciphertext)
}

struct StatefulCBC {
    key: Vec<u8>,
    last_block: Vec<u8>,
}

impl StatefulCBC {
    fn new(key: &[u8], iv: Option<Vec<u8>>) -> Self {
        Self {
            key: key.to_vec(),
            last_block: iv.unwrap_or_else(|| vec![0; 16]),
        }
    }

    fn encrypt_block(&mut self, block: &[u8]) -> OpenSSLResult<Vec<u8>> {
        let xord = chall02::xor(block, &self.last_block);
        let result = encrypt_aes_ecb(&xord, &self.key)?;
        self.last_block = result.clone();

        Ok(result)
    }

    fn decrypt_block(&mut self, block: &[u8]) -> OpenSSLResult<Vec<u8>> {
        let result = chall07::decrypt_aes_ecb(block, &self.key)?;
        let xord = chall02::xor(&result, &self.last_block);
        self.last_block = block.to_vec();

        Ok(xord)
    }
}

pub fn encrypt_aes_cbc(
    plaintext: &[u8],
    key: &[u8],
    iv: Option<Vec<u8>>,
) -> OpenSSLResult<Vec<u8>> {
    let mut state = StatefulCBC::new(key, iv);
    plaintext
        .chunks(16)
        .map(|block| chall09::pad(block, 16))
        .map(|block| state.encrypt_block(&block))
        .flatten_ok()
        .collect()
}

pub fn decrypt_aes_cbc(ciphertext: &[u8], key: &[u8]) -> OpenSSLResult<Vec<u8>> {
    let mut state = StatefulCBC::new(key, None);
    ciphertext
        .chunks(16)
        .map(|block| chall09::pad(block, 16))
        .map(|block| state.decrypt_block(&block))
        .flatten_ok()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let reader = util::get_file_reader("data/10.txt");
        let input = io::read_to_string(reader).expect("Failed to read file");
        let ciphertext = chall06::base64_decode(&input).expect("Failed to base64-decode");
        let key = "YELLOW SUBMARINE".as_bytes();
        let plaintext = decrypt_aes_cbc(&ciphertext, key).expect("Failed to decrypt");

        // message is too long to check the entire thing here; this is a heuristic
        let msg_needle = "Play that funky music".as_bytes();
        assert_eq!(6, util::count_occurrences(&plaintext, msg_needle));
    }

    #[test]
    fn encrypt_aes_ecb_works() {
        let plaintext = "The quick brown?".as_bytes();
        let key = "secret (16bytes)".as_bytes();
        assert_eq!(
            plaintext,
            chall07::decrypt_aes_ecb(&encrypt_aes_ecb(plaintext, key).unwrap(), key).unwrap()
        )
    }

    #[test]
    fn encrypt_aes_cbc_then_decrypt_works() {
        let plaintext = "what even is 16b among friends??".as_bytes();
        let key = "here are 16b sir".as_bytes();
        let ciphertext = encrypt_aes_cbc(plaintext, key, None).unwrap();
        let result = decrypt_aes_cbc(&ciphertext, key).unwrap();

        assert_eq!(util::bytes_to_hex(plaintext), util::bytes_to_hex(&result));
    }
}
