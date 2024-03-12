use itertools::Itertools;

use crate::util;

pub fn solve_chall15() {
    let input = util::read_one_line();
    let output = unpad(input.as_bytes()).expect("Invalid padding");
    println!("{}", String::from_utf8(output).expect("Invalid UTF-8"))
}

// padding is required!!
pub fn unpad(text: &[u8]) -> Result<Vec<u8>, ()> {
    if text.len() % 16 != 0 {
        return Err(());
    }

    let padding_length = *text.last().ok_or(())?;
    if !(1..=16).contains(&padding_length) || text.len() < padding_length.into() {
        return Err(());
    }

    if text
        .iter()
        .rev()
        .take(padding_length.into())
        .any(|x| *x != padding_length)
    {
        return Err(());
    }

    Ok(text
        .iter()
        .take(text.len() - (padding_length as usize))
        .copied()
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        assert_eq!(
            "ICE ICE BABY",
            String::from_utf8(unpad("ICE ICE BABY\x04\x04\x04\x04".as_bytes()).unwrap()).unwrap()
        );
        assert_eq!(
            "YELLOW SUBMARINE",
            String::from_utf8(
                unpad(
                    "YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10"
                        .as_bytes()
                )
                .unwrap()
            )
            .unwrap()
        );
    }

    #[test]
    fn detect_invalid() {
        assert_eq!(Err(()), unpad("ICE ICE BABY\x05\x05\x05\x05".as_bytes()));
        assert_eq!(Err(()), unpad("ICE ICE BABY\x01\x02\x03\x04".as_bytes()));
    }
}
