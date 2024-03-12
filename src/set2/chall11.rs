use itertools::Itertools;
use rand::{self, Rng};

use super::chall10;

pub fn solve_chall11() {
    if aes_ecb_detector(&aes_encryption_oracle).expect("Failed to detect") {
        println!("Mode: ECB");
    } else {
        println!("Mode: CBC");
    }
}

pub fn random_aes_key() -> Vec<u8> {
    let bytes: [u8; 16] = rand::random();

    bytes.to_vec()
}

static mut ORACLE_CHOICE: bool = false;

fn aes_encryption_oracle(plaintext: &[u8]) -> chall10::OpenSSLResult<Vec<u8>> {
    let key = random_aes_key();

    let mut wrapped = vec![];
    for _ in 0..rand::thread_rng().gen_range(5..=10) {
        wrapped.push(rand::random());
    }

    wrapped.extend(plaintext);
    for _ in 0..rand::thread_rng().gen_range(5..=10) {
        wrapped.push(rand::random());
    }

    if rand::random() {
        println!("Oracle: ECB");
        unsafe {
            ORACLE_CHOICE = true;
        }
        chall10::encrypt_aes_ecb(&wrapped, &key)
    } else {
        println!("Oracle: CBC");
        let iv: [u8; 16] = rand::random();
        chall10::encrypt_aes_cbc(&wrapped, &key, Some(iv.to_vec()))
    }
}

pub fn aes_ecb_detector<F>(f: &F) -> chall10::OpenSSLResult<bool>
where
    F: Fn(&[u8]) -> chall10::OpenSSLResult<Vec<u8>>,
{
    let plaintext = [0; 64];
    let ciphertext = f(&plaintext)?;

    // 2 equal consecutive blocks
    Ok(ciphertext.chunks(16).tuple_windows().any(|(x, y)| x == y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = aes_ecb_detector(&aes_encryption_oracle).expect("Failed to detect");
        let expected = unsafe { ORACLE_CHOICE };

        assert_eq!(expected, result);
    }
}
