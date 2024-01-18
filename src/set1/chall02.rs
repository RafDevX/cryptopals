use crate::util;

pub fn solve_chall02() {
    let input = util::read_hex_lines();
    let a = input.first().expect("No first operand provided");
    let b = input.get(1).expect("No second operand provided");
    let output = xor(a, b);
    println!("{}", util::bytes_to_hex(&output))
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.len() != b.len() {
        panic!("Operators have different length")
    }

    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        assert_eq!(
            "746865206b696420646f6e277420706c6179",
            util::bytes_to_hex(&xor(
                &util::hex_to_bytes("1c0111001f010100061a024b53535009181c"),
                &util::hex_to_bytes("686974207468652062756c6c277320657965")
            ))
        )
    }
}
