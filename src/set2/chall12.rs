use std::iter;

use itertools::Itertools;

use crate::set1::chall06;

use super::{chall10, chall11};

pub fn solve_chall12() {
    let result =
        discover_unknown_suffix(aes_consistent_encryption_oracle).expect("Failed to discover");
    let output = String::from_utf8(result).expect("Invalid UTF-8");
    println!("{output}");
}

static mut ORACLE_KEY: Option<Vec<u8>> = None;

fn aes_consistent_encryption_oracle(plaintext: &[u8]) -> chall10::OpenSSLResult<Vec<u8>> {
    let key = unsafe {
        if ORACLE_KEY.is_none() {
            ORACLE_KEY = Some(chall11::random_aes_key());
        }
        ORACLE_KEY.as_ref().unwrap()
    };

    let mut plaintext = plaintext.to_vec();
    let unknown = chall06::base64_decode(concat!(
        "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg",
        "aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq",
        "dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg",
        "YnkK"
    ))
    .unwrap();
    plaintext.extend(unknown);

    chall10::encrypt_aes_ecb(&plaintext, key)
}

fn discover_unknown_suffix<F>(f: F) -> chall10::OpenSSLResult<Vec<u8>>
where
    F: Fn(&[u8]) -> chall10::OpenSSLResult<Vec<u8>>,
{
    let (block_size, secret_len) = find_block_size(&f)?;
    if !chall11::aes_ecb_detector(&f)? {
        unimplemented!("Only ECB is supported")
    };

    let mut secret = vec![];

    while secret.len() < secret_len {
        let output_base = iter::repeat(0x72)
            .take(
                block_size
                    .saturating_sub(secret.len() % block_size)
                    .saturating_sub(1),
            )
            .collect_vec();
        let output = f(&output_base)?;

        let mut base = iter::repeat(0x72)
            .take(block_size.saturating_sub(secret.len()).saturating_sub(1))
            .collect_vec();
        base.extend(secret[secret.len().saturating_sub(block_size - 1)..].iter());

        let block_index = secret.len() / block_size;

        for i in u8::MIN..=u8::MAX {
            let mut candidate = base.clone();
            candidate.push(i);
            let result = f(&candidate)?;

            let output_block = &output[block_index * block_size..][..block_size];
            let result_block = &result[..block_size];
            if output_block == result_block {
                secret.push(i);
                break;
            }
        }
    }

    Ok(secret)
}

pub fn find_block_size<F>(f: &F) -> chall10::OpenSSLResult<(usize, usize)>
where
    F: Fn(&[u8]) -> chall10::OpenSSLResult<Vec<u8>>,
{
    let mut i = 1;
    let mut output_size = None;
    let block_size = loop {
        let plaintext = iter::repeat(0x72).take(i).collect_vec();
        let output = f(&plaintext)?;

        match output_size {
            None => output_size = Some(output.len()),
            Some(size) if size < output.len() => break output.len() - size,
            Some(_) => {}
        }
        i += 1;
    };

    let secret_len = output_size.unwrap() - i + 1;

    Ok((block_size, secret_len))
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
