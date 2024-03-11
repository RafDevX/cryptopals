use crate::util;

pub fn solve_chall09() {
    let input = util::read_one_line();
    let output = pad(input.as_bytes(), 20);
    println!("{}", String::from_utf8(output).expect("Invalid UTF-8"))
}

fn pad(text: &[u8], target_length: usize) -> Vec<u8> {
    if text.len() > target_length {
        panic!("Text too long for {target_length}!");
    };

    let missing = (target_length - text.len())
        .try_into()
        .expect("Difference cannot be represented!");

    text.iter()
        .copied()
        .chain(std::iter::repeat(missing))
        .take(target_length)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        assert_eq!(
            "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes(),
            pad("YELLOW SUBMARINE".as_bytes(), 20)
        )
    }
}
