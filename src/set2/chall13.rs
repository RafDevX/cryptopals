use std::collections::HashMap;

use itertools::Itertools;

use crate::set1::chall07;

use super::{chall10, chall11, chall15};

pub fn solve_chall13() {
    let result = make_admin_profile(encrypt_profile_for, decrypt_profile).unwrap();
    println!("{result:?}")
}

fn parse_cookie(cookie: &[u8]) -> HashMap<String, String> {
    let mut obj = HashMap::new();

    for (sep, mut kv) in &cookie.iter().group_by(|x| **x == b'&') {
        if sep {
            continue;
        }
        let mut key = String::new();
        while let Some(&c) = kv.next() {
            if c == b'=' {
                obj.insert(key, kv.copied().map(char::from).collect());
                break;
            }

            key.push(char::from(c));
        }
    }

    obj
}

fn profile_for(email: &[u8]) -> Vec<u8> {
    let mut result = vec![];
    result.extend("email=".as_bytes());
    result.extend(email.iter().filter(|x| **x != b'&' && **x != b'='));
    result.extend("&uid=10&role=user".as_bytes());

    result
}

static mut ORACLE_KEY: Option<Vec<u8>> = None;

fn get_oracle_key() -> &'static Vec<u8> {
    unsafe {
        if ORACLE_KEY.is_none() {
            ORACLE_KEY = Some(chall11::random_aes_key());
        }
        ORACLE_KEY.as_ref().unwrap()
    }
}

fn encrypt_profile_for(email: &[u8]) -> chall10::OpenSSLResult<Vec<u8>> {
    let profile = profile_for(email);
    chall10::encrypt_aes_ecb(&profile, get_oracle_key())
}

fn decrypt_profile(ciphertext: &[u8]) -> chall10::OpenSSLResult<HashMap<String, String>> {
    let cookie = chall07::decrypt_aes_ecb(ciphertext, get_oracle_key())?;
    let cookie = chall15::unpad(&cookie).unwrap();

    Ok(parse_cookie(&cookie))
}

fn make_admin_profile<E, D>(
    encrypter: E,
    decrypter: D,
) -> chall10::OpenSSLResult<HashMap<String, String>>
where
    E: Fn(&[u8]) -> chall10::OpenSSLResult<Vec<u8>>,
    D: Fn(&[u8]) -> chall10::OpenSSLResult<HashMap<String, String>>,
{
    // ..........."d=10&role=" || "admin"...padding
    let payload = "xxxxxxxxxxadmin\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0bxxx";

    let mut ciphertext = encrypter(payload.as_bytes())?;
    ciphertext.copy_within(16..(2 * 16), 3 * 16);
    ciphertext.copy_within((2 * 16).., 16);
    ciphertext.truncate(ciphertext.len().saturating_sub(16));

    decrypter(&ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = make_admin_profile(encrypt_profile_for, decrypt_profile).unwrap();
        assert_eq!(Some(&"admin".to_owned()), result.get(&"role".to_owned()));
    }

    #[test]
    fn parse_cookie_works() {
        let result = parse_cookie("foo=bar&baz=qux&zap=zazzle".as_bytes());
        assert_eq!(3, result.len());
        assert_eq!(
            Some("bar".to_owned()),
            result.get(&"foo".to_owned()).cloned()
        );
        assert_eq!(
            Some("qux".to_owned()),
            result.get(&"baz".to_owned()).cloned()
        );
        assert_eq!(
            Some("zazzle".to_owned()),
            result.get(&"zap".to_owned()).cloned()
        );
    }

    #[test]
    fn profile_for_works() {
        let result: String = profile_for("a&b=c@example.com".as_bytes())
            .iter()
            .copied()
            .map(char::from)
            .collect();
        assert_eq!("email=abc@example.com&uid=10&role=user", result);
    }
}
