use std::iter;

use itertools::Itertools;
use rand::Rng;

use crate::set1::chall06;

use super::{chall10, chall11, chall12};

pub fn solve_chall14() {
    let result =
        discover_unknown_suffix(aes_consistent_encryption_oracle).expect("Failed to discover");
    let output = String::from_utf8(result).expect("Invalid UTF-8");
    println!("{output}");
}

static mut ORACLE_PARAMS: Option<(Vec<u8>, Vec<u8>)> = None;

fn aes_consistent_encryption_oracle(plaintext: &[u8]) -> chall10::OpenSSLResult<Vec<u8>> {
    let (key, prefix) = unsafe {
        if ORACLE_PARAMS.is_none() {
            let key = chall11::random_aes_key();
            let mut prefix = vec![];
            for _ in 0..rand::thread_rng().gen_range(5..=128) {
                prefix.push(rand::random());
            }

            ORACLE_PARAMS = Some((key, prefix));
        }
        ORACLE_PARAMS.as_ref().unwrap()
    };

    let mut obfuscated = prefix.clone();
    obfuscated.extend(plaintext);

    let unknown = chall06::base64_decode(concat!(
        "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg",
        "aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq",
        "dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg",
        "YnkK"
    ))
    .unwrap();
    obfuscated.extend(unknown);

    chall10::encrypt_aes_ecb(&obfuscated, key)
}

fn discover_unknown_suffix<F>(f: F) -> chall10::OpenSSLResult<Vec<u8>>
where
    F: Fn(&[u8]) -> chall10::OpenSSLResult<Vec<u8>>,
{
    let (block_size, prefix_plus_secret_len) = chall12::find_block_size(&f)?;
    if !chall11::aes_ecb_detector(&f)? {
        unimplemented!("Only ECB is supported")
    };

    let prefix_length = find_unknown_prefix_length(&f, block_size)?;
    let prefix_blocks = prefix_length / block_size + 1;
    let payload_offset = block_size - (prefix_length % block_size);

    let mut secret = vec![];

    while secret.len() + prefix_length < prefix_plus_secret_len {
        let output_base = iter::repeat(0x72)
            .take(
                (block_size + payload_offset)
                    .saturating_sub(secret.len() % block_size)
                    .saturating_sub(1),
            )
            .collect_vec();
        let output = f(&output_base)?;

        let mut base = iter::repeat(0x72)
            .take(
                (block_size + payload_offset)
                    .saturating_sub(secret.len())
                    .saturating_sub(1),
            )
            .collect_vec();
        base.extend(
            secret[secret
                .len()
                .saturating_sub(block_size - 1)
                .saturating_sub(payload_offset)..]
                .iter(),
        );

        let block_index = secret.len() / block_size;

        for i in u8::MIN..=u8::MAX {
            let mut candidate = base.clone();
            candidate.push(i);
            let result = f(&candidate)?;

            let output_block = &output[(block_index + prefix_blocks) * block_size..][..block_size];
            let result_block = &result[prefix_blocks * block_size..][..block_size];
            if output_block == result_block {
                secret.push(i);
                break;
            }
        }
    }

    Ok(secret)
}

fn find_unknown_prefix_length<F>(f: &F, block_size: usize) -> chall10::OpenSSLResult<usize>
where
    F: Fn(&[u8]) -> chall10::OpenSSLResult<Vec<u8>>,
{
    let mut i = block_size * 2;

    loop {
        let plaintext = iter::repeat(0x72).take(i).collect_vec();
        let ciphertext = f(&plaintext)?;
        if let Some(pos) = ciphertext
            .chunks(block_size)
            .tuple_windows()
            .position(|(x, y)| x == y)
        {
            // 2 consecutive equal blocks
            break Ok(pos * block_size - (i % block_size));
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = discover_unknown_suffix(aes_consistent_encryption_oracle).unwrap();

        assert_eq!(
            concat!(
                "Rollin' in my 5.0\nWith my rag-top down so my hair can blow\n",
                "The girlies on standby waving just to say hi\n",
                "Did you stop? No, I just drove by\n"
            ),
            String::from_utf8(result).unwrap()
        )
    }
}
